- [ ] Local UI: make usable
    - [ ] UI improvements

- [ ] Logging: log all incoming requests to stdout
- [ ] TUI Usage
    - [ ] Give option for `htop` like TUI for non-web development
- [ ] Improve Port selection
    - port-ranges: Pick first available if first is busy

- [ ] Improve proxy
    - Support large requests (Downloads and video/audio streams)
        - Support streaming requests
        - Make requests exactly the same way as the client is making
            - multipart or whatever
        - which can't fit in memory
    - Document all the headers modified
    - Maybe support redirects in better way
    - Reduce the amount of headers being modified
    - Add extra headers to store previous headers

- [ ] Socks5 Proxy integration
- [ ] Support HTTP2 and HTTP3

- What is interesting for user:
    - The request sent by client to proxy_server
    - The response sent by server to proxy
    - What is not interesting, althought these will be mostly same
        - Request sent by proxy to server
        - Response sent by proxy to client
