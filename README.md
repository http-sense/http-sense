# HTTP Sense

HTTP Sense is a reverse proxy and network monitoring tool that provides comprehensive network monitoring for your backend servers and microservices. It provides a secure and high-performance gateway that can be used to monitor incoming traffic. HTTP Sense's features include real-time traffic analysis, application-level filtering, user authentication, and more.

**HTTP Sense is built on Supabase and utilizes the following features:**
- Supabase Postgres database - as the primary database
- Supabase Authentication - for RBAC & ABAC
- Supabase Realtime - To listen to realtime DB changes on the HTTP Sense UI

**You can use HTTP Sense for the following:**
- As a network monitoring tool for your production environment (using --publish flag)
- As a reverse proxy for your server in production or even, other lower environments
- As a debugging/monitoring tool during development, like the Browser DevTools are for the browser
- Just for fun

# References
- Video Demo: https://www.youtube.com/watch?v=Qvx4-iaqDq4
- Homepage: https://www.httpsense.com  
- Crates: https://crates.io/crates/http-sense  
- Git Repository: https://github.com/http-sense/http-sense  

# Setting up the repository for development
You will need `cargo` in order to install and run the HTTP Sense CLI. If you do not have cargo installed on your system, please follow the below guide from the official rust-lang docs:

> https://doc.rust-lang.org/cargo/getting-started/installation.html

Once you have `cargo` installed, you can spin up the dev server using the following command:
```bash
cargo run -- <YOUR_SERVER_URL> --publish 
```
Usage Example:
```bash
cargo run -- https://0caf7838-7d0a-4a56-8b04-65a9c6f5815e.mock.pstmn.io --publish
```

- When using the --publish flag, you won't need to run the UI server separately and instead can use the production UI at https://www.httpsense.com
- Once the server has started, it will start intercepting and reverse proxying every request that comes in to your actual server and publishes the monitoring stats to the URL that is provided to you via the CLI. **Please make sure you copy the URL with the hash when opening it in the browser.**

example: 
```bash
https://www.httpsense.com/526a0f41-a617-4f9e-8b40-f18d40104f99/#YXN5MW9wdE1VcXVWS0E1MHY3TDdDa0hJT0RyOVZqQGV4YW1wbGUuY29tOjp1Ym56Y0FOeEliM0toajdRazU5bW4xWFgwNVBkNnE=
```

# Installation

If you do not have `cargo` installed, please refer to the above section. Once you have it installed, proceed with the installation for HTTP Sense using the following command:

```bash
# To get the latest and greatest version
cargo install --git https://github.com/http-sense/http-sense
```
After installing HTTP Sense, you can run the CLI server using the commands in the next section. 

# CLI Commands & Usage
```bash
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

      --api-port <API_PORT>
          Port at which api server should listen (Alpha)

          [default: 6101]

      --api-addr <API_ADDR>
          Address that api server should bind to

          [default: 127.0.0.1]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information

```

# Contributing
If you're looking to contribute to HTTP Sense, please read and follow the guidelines from [CONTRIBUTING.md](https://github.com/http-sense/http-sense/blob/main/CONTRIBUTING.md).

# Maintainers
<a href="https://github.com/http-sense/http-sense/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=http-sense/http-sense" />
</a>


