# frozen_string_literal: true

require 'net/http'
require 'uri'

module Mob
  # Net::HTTP-based implementation of the Rust HttpTransport trait.
  #
  # Uses Ruby's OpenSSL bindings which properly handle the system CA
  # bundle, avoiding the UnknownIssuer errors that occur with
  # rustls-native-certs on some platforms.
  class NativeHttpTransport
    include Mob::HttpTransport

    def post(url, body)
      uri = URI.parse(url)
      http = Net::HTTP.new(uri.host, uri.port)
      http.use_ssl = (uri.scheme == 'https')

      request = Net::HTTP::Post.new(uri.request_uri)
      request['Content-Type'] = 'application/json'
      request.body = body.pack('C*')

      response = http.request(request)

      code = response.code.to_i
      unless (200..299).include?(code)
        raise Mob::TransportError::RequestFailed.new(
          "HTTP #{code}: #{response.body}"
        )
      end

      response.body.bytes
    rescue Mob::TransportError
      raise
    rescue SocketError, Errno::ECONNREFUSED, Errno::ETIMEDOUT,
           Net::OpenTimeout, Net::ReadTimeout, OpenSSL::SSL::SSLError => e
      raise Mob::TransportError::NetworkError.new(e.message)
    rescue StandardError => e
      raise Mob::TransportError::RequestFailed.new(e.message)
    end
  end
end
