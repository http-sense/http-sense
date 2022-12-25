- [ ] Local UI: get it working with svelte
    - cors setup
- [ ] Logging: log all incoming requests to cli
- [ ] Improve Port selection
    - port-ranges: Pick first available if first is busy

- [ ] Improve proxy
    - Reduce the amount of headers being modified
    - Add extra headers to store previous headers
    - Document all the headers modified
    - Support streaming requests

- [ ] Socks5 Proxy integration
- [ ] Support HTTP2 and HTTP3

- What is interesting for user:
    - The request sent by client to proxy_server
    - The response sent by server to proxy
    - What is not interesting, althought these will be mostly same
        - Request sent by proxy to server
        - Response sent by proxy to client
