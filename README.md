# HTTP Sense

HTTP Sense is a `reverse-proxy + network monitor` for your http server. 


**You can use this to:**
- As a monitoring tool for prod (using --publish command)
- Debug your backend server in development
- Just for fun

# Installation
```
cargo install http-sense
```

# CLI Usage
```
Usage: http-sense [OPTIONS] <ORIGIN_URL>

TLDR:
   http-sense http://localhost:8004 --publish

   # use port number as short-hand for localhost servers
   http-sense 8004 --publish            

   http-sense httpsense.com --proxy-port 6001 --publish

   http-sense http://localhost:8004 --proxy-port 6001 --proxy-addr 0.0.0.0


Options:
      --publish
          Publish requests to supabase db, allowing you to remotely access request details

  -p, --proxy-port <PROXY_PORT>
          Port at which proxy server should listen

          [default: 6100]

  -a, --proxy-addr <PROXY_ADDR>
          Address that proxy server should bind to

          [default: 127.0.0.1]

      --ui-port <UI_PORT>
          Port at which ui server should listen (Alpha)

          [default: 6101]

      --ui-addr <UI_ADDR>
          Address that ui server should bind to

          [default: 127.0.0.1]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information


```
