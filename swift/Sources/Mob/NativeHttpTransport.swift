import Foundation

/// URLSession-based implementation of the Rust `HttpTransport` trait.
///
/// Uses the platform's native TLS stack (Security.framework) which properly
/// handles the system CA bundle, avoiding the `UnknownIssuer` errors that
/// occur with rustls-native-certs on iOS.
public final class NativeHttpTransport: HttpTransport, @unchecked Sendable {
    public init() {}

    public func post(url: String, body: Data) throws -> Data {
        guard let requestUrl = URL(string: url) else {
            throw TransportError.RequestFailed(message: "Invalid URL: \(url)")
        }

        var request = URLRequest(url: requestUrl)
        request.httpMethod = "POST"
        request.httpBody = body
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        return try performRequest(request)
    }

    public func get(url: String) throws -> Data {
        guard let requestUrl = URL(string: url) else {
            throw TransportError.RequestFailed(message: "Invalid URL: \(url)")
        }

        let request = URLRequest(url: requestUrl)
        return try performRequest(request)
    }

    private func performRequest(_ request: URLRequest) throws -> Data {
        var result: Swift.Result<Data, Error>!
        let semaphore = DispatchSemaphore(value: 0)

        URLSession.shared.dataTask(with: request) { data, _, error in
            if let error = error {
                result = .failure(
                    TransportError.NetworkError(message: error.localizedDescription)
                )
            } else if let data = data {
                result = .success(data)
            } else {
                result = .failure(
                    TransportError.RequestFailed(message: "No data in response")
                )
            }
            semaphore.signal()
        }.resume()

        semaphore.wait()
        return try result.get()
    }
}
