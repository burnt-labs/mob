package com.burnt.mob

import uniffi.mob.HttpTransport
import uniffi.mob.TransportError
import java.io.IOException
import java.net.HttpURLConnection
import java.net.URI

/**
 * HttpURLConnection-based implementation of the Rust `HttpTransport` trait.
 *
 * Uses the JVM/Android platform's native TLS stack, avoiding the
 * `UnknownIssuer` errors that occur with rustls-native-certs on Android.
 */
class NativeHttpTransport : HttpTransport {
    override fun post(url: String, body: List<UByte>): List<UByte> {
        val connection = openConnection(url)
        try {
            connection.requestMethod = "POST"
            connection.doOutput = true
            connection.setRequestProperty("Content-Type", "application/json")

            val bodyBytes = ByteArray(body.size) { body[it].toByte() }
            connection.outputStream.use { it.write(bodyBytes) }

            return readResponse(connection)
        } catch (e: TransportError) {
            throw e
        } catch (e: IOException) {
            throw TransportError.NetworkError(e.message ?: "Unknown network error")
        } catch (e: Exception) {
            throw TransportError.RequestFailed(e.message ?: "Unknown error")
        } finally {
            connection.disconnect()
        }
    }

    override fun get(url: String): List<UByte> {
        val connection = openConnection(url)
        try {
            connection.requestMethod = "GET"
            return readResponse(connection)
        } catch (e: TransportError) {
            throw e
        } catch (e: IOException) {
            throw TransportError.NetworkError(e.message ?: "Unknown network error")
        } catch (e: Exception) {
            throw TransportError.RequestFailed(e.message ?: "Unknown error")
        } finally {
            connection.disconnect()
        }
    }

    private fun openConnection(url: String): HttpURLConnection {
        return try {
            URI(url).toURL().openConnection() as HttpURLConnection
        } catch (e: Exception) {
            throw TransportError.RequestFailed("Invalid URL: $url")
        }
    }

    private fun readResponse(connection: HttpURLConnection): List<UByte> {
        val responseCode = connection.responseCode
        if (responseCode !in 200..299) {
            val errorBody = try {
                connection.errorStream?.readBytes()?.decodeToString() ?: ""
            } catch (_: Exception) { "" }
            throw TransportError.RequestFailed("HTTP $responseCode: $errorBody")
        }
        return connection.inputStream.use { it.readBytes() }.map { it.toUByte() }
    }
}
