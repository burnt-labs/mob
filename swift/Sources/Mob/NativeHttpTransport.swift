import Foundation
import Network
import os.log

private let httpLog = Logger(subsystem: "xyz.burnt.mob", category: "NativeHttp")

/// HttpTransport implementation using NWConnection for synchronous HTTP/1.1 POST.
///
/// Uses TCP+TLS directly (no QUIC/HTTP3) to avoid iOS simulator QUIC issues
/// with Cloudflare-fronted endpoints. The Rust FFI calls `post()` from a
/// background thread, so it must be synchronous (blocks via DispatchSemaphore).
public final class NativeHttpTransport: HttpTransport, @unchecked Sendable {
    public init() {}

    public func post(url: String, body: Data) throws -> Data {
        guard let parsedUrl = URL(string: url),
              let host = parsedUrl.host else {
            throw TransportError.RequestFailed(message: "Invalid URL: \(url)")
        }

        let port = UInt16(parsedUrl.port ?? (parsedUrl.scheme == "https" ? 443 : 80))
        let usesTLS = parsedUrl.scheme == "https"

        // TCP + TLS only — no QUIC/HTTP3
        let params: NWParameters
        if usesTLS {
            let tlsOptions = NWProtocolTLS.Options()
            sec_protocol_options_add_tls_application_protocol(
                tlsOptions.securityProtocolOptions, "http/1.1"
            )
            params = NWParameters(tls: tlsOptions, tcp: NWProtocolTCP.Options())
        } else {
            params = NWParameters(tls: nil, tcp: NWProtocolTCP.Options())
        }

        let connection = NWConnection(
            host: .name(host, nil),
            port: .init(rawValue: port)!,
            using: params
        )

        let semaphore = DispatchSemaphore(value: 0)
        var result: Result<Data, TransportError> = .failure(.RequestFailed(message: "Connection timed out"))

        connection.stateUpdateHandler = { state in
            switch state {
            case .ready:
                self.sendRequest(connection: connection, host: host, path: parsedUrl.path, body: body) { res in
                    result = res
                    semaphore.signal()
                }
            case .failed(let error):
                httpLog.error("[NativeHttp] connection failed: \(error)")
                result = .failure(.NetworkError(message: error.localizedDescription))
                semaphore.signal()
            default:
                break
            }
        }

        let queue = DispatchQueue(label: "xyz.burnt.mob.http", qos: .userInitiated)
        connection.start(queue: queue)

        let timeout = DispatchTime.now() + .seconds(30)
        if semaphore.wait(timeout: timeout) == .timedOut {
            connection.cancel()
            throw TransportError.NetworkError(message: "Request timed out after 30s")
        }

        connection.cancel()

        switch result {
        case .success(let data):
            return data
        case .failure(let error):
            throw error
        }
    }

    // MARK: - Private

    private func sendRequest(
        connection: NWConnection,
        host: String,
        path: String,
        body: Data,
        completion: @escaping (Result<Data, TransportError>) -> Void
    ) {
        let requestPath = path.isEmpty ? "/" : path
        var header = "POST \(requestPath) HTTP/1.1\r\n"
        header += "Host: \(host)\r\n"
        header += "Content-Type: application/json\r\n"
        header += "Content-Length: \(body.count)\r\n"
        header += "Connection: close\r\n"
        header += "\r\n"

        var requestData = header.data(using: .utf8)!
        requestData.append(body)

        connection.send(content: requestData, completion: .contentProcessed { error in
            if let error = error {
                httpLog.error("[NativeHttp] send error: \(error)")
                completion(.failure(.NetworkError(message: error.localizedDescription)))
                return
            }
            self.readResponse(connection: connection, buffer: Data(), completion: completion)
        })
    }

    private func readResponse(
        connection: NWConnection,
        buffer: Data,
        completion: @escaping (Result<Data, TransportError>) -> Void
    ) {
        connection.receive(minimumIncompleteLength: 1, maximumLength: 65536) { data, _, isComplete, error in
            var accumulated = buffer
            if let data = data {
                accumulated.append(data)
            }

            if isComplete || error != nil {
                // Connection closed or errored — parse what we have
                if let bodyData = self.extractResponseBody(from: accumulated) {
                    completion(.success(bodyData))
                } else if accumulated.isEmpty {
                    let msg = error?.localizedDescription ?? "No response data"
                    completion(.failure(.NetworkError(message: msg)))
                } else {
                    completion(.failure(.RequestFailed(message: "Failed to parse HTTP response")))
                }
                return
            }

            // Check if we have the full response based on Content-Length
            if let bodyData = self.extractCompleteBody(from: accumulated) {
                completion(.success(bodyData))
                return
            }

            // Need more data
            self.readResponse(connection: connection, buffer: accumulated, completion: completion)
        }
    }

    private func extractResponseBody(from data: Data) -> Data? {
        let separator: [UInt8] = [0x0D, 0x0A, 0x0D, 0x0A]
        guard let range = data.range(of: Data(separator)) else { return nil }
        let bodyStart = range.upperBound
        guard bodyStart <= data.count else { return nil }

        let headerData = data.subdata(in: data.startIndex..<range.lowerBound)
        let headers = String(data: headerData, encoding: .utf8)?.lowercased() ?? ""
        let rawBody = data.subdata(in: bodyStart..<data.count)

        // Decode chunked transfer encoding if needed
        if headers.contains("transfer-encoding: chunked") || headers.contains("transfer-encoding:chunked") {
            return decodeChunked(rawBody)
        }
        return rawBody
    }

    /// Decode HTTP chunked transfer encoding: size\r\ndata\r\nsize\r\ndata\r\n0\r\n\r\n
    private func decodeChunked(_ data: Data) -> Data? {
        var result = Data()
        var offset = 0
        let bytes = [UInt8](data)
        let crlf: [UInt8] = [0x0D, 0x0A]

        while offset < bytes.count {
            guard let crlfIndex = findSequence(crlf, in: bytes, from: offset) else { break }
            let sizeLine = String(bytes: bytes[offset..<crlfIndex], encoding: .ascii)?.trimmingCharacters(in: .whitespaces) ?? ""
            let sizeHex = sizeLine.components(separatedBy: ";").first ?? sizeLine
            guard let chunkSize = UInt(sizeHex, radix: 16) else { break }
            if chunkSize == 0 { break }

            let chunkStart = crlfIndex + 2
            let chunkEnd = chunkStart + Int(chunkSize)
            guard chunkEnd <= bytes.count else { break }

            result.append(contentsOf: bytes[chunkStart..<chunkEnd])
            offset = chunkEnd + 2
        }
        return result.isEmpty ? nil : result
    }

    private func findSequence(_ seq: [UInt8], in bytes: [UInt8], from start: Int) -> Int? {
        guard seq.count > 0, start + seq.count <= bytes.count else { return nil }
        for i in start...(bytes.count - seq.count) {
            if bytes[i..<(i + seq.count)].elementsEqual(seq) { return i }
        }
        return nil
    }

    private func extractCompleteBody(from data: Data) -> Data? {
        let separator: [UInt8] = [0x0D, 0x0A, 0x0D, 0x0A]
        guard let range = data.range(of: Data(separator)) else { return nil }

        let headerData = data.subdata(in: data.startIndex..<range.lowerBound)
        guard let headers = String(data: headerData, encoding: .utf8)?.lowercased() else { return nil }

        for line in headers.components(separatedBy: "\r\n") {
            if line.hasPrefix("content-length:") {
                let value = line.dropFirst("content-length:".count).trimmingCharacters(in: .whitespaces)
                if let contentLength = Int(value) {
                    let bodyStart = range.upperBound
                    let bodyEnd = bodyStart + contentLength
                    if data.count >= bodyEnd {
                        return data.subdata(in: bodyStart..<bodyEnd)
                    }
                }
                break
            }
        }

        return nil
    }
}
