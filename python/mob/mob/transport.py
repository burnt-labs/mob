"""
Platform-native HTTP transport using urllib.

Uses the system's TLS stack (via Python's ssl module) which properly
handles the OS CA bundle, avoiding the UnknownIssuer errors that
occur with rustls-native-certs on some platforms.
"""

import urllib.request
import urllib.error

from .mob import HttpTransport, TransportError


class NativeHttpTransport(HttpTransport):
    """urllib-based implementation of the Rust HttpTransport trait."""

    def post(self, url: str, body: bytes) -> bytes:
        req = urllib.request.Request(
            url,
            data=body,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        return self._perform(req)

    def get(self, url: str) -> bytes:
        req = urllib.request.Request(url, method="GET")
        return self._perform(req)

    def _perform(self, req: urllib.request.Request) -> bytes:
        try:
            with urllib.request.urlopen(req) as resp:
                return resp.read()
        except urllib.error.HTTPError as e:
            error_body = ""
            try:
                error_body = e.read().decode()
            except Exception:
                pass
            raise TransportError.RequestFailed(
                f"HTTP {e.code}: {error_body}"
            )
        except urllib.error.URLError as e:
            raise TransportError.NetworkError(str(e.reason))
        except Exception as e:
            raise TransportError.RequestFailed(str(e))
