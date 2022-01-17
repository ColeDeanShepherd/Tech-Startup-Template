# Tech-Startup-Template

## Running the client

1. `cd` into `client`
2. Run `npm install`
3. Run `npm start`

## Setting up the server for development

1. Install [PostgreSQL](https://www.postgresql.org/download/)
2. If on Windows, add `PQ_LIB_DIR=C:\Program Files\PostgreSQL\14\lib` to system environment variables, add `C:\Program Files\PostgreSQL\14\lib` and `C:\Program Files\PostgreSQL\14\bin` to `PATH`. See [this blog post](https://dev.to/ssivakumar/rust-diesel-fixing-issues-with-setup-3k56) for more info.
3. `cargo install diesel_cli --no-default-features --features postgres`

## Running the server

If on Windows:

1. Install [vcpkg](https://vcpkg.io/en/getting-started.html)
2. Run `vcpkg install openssl:x64-windows-static-md`

1. `cd` into `server`
2. run `cargo run`
3. Navigate to [https://localhost:8080/](https://localhost:8080/) in your browser.

NOTE: The SSL key was generated with the following command: `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -sha256 -subj '/CN=localhost'`