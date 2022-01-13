# Tech-Startup-Template

## Running the server

If on Windows:

1. Install [vcpkg](https://vcpkg.io/en/getting-started.html)
2. Run `vcpkg install openssl:x64-windows-static-md`

1. `cd` into `server`
2. run `cargo run`
3. Navigate to [https://localhost:8080/](https://localhost:8080/) in your browser.

NOTE: The SSL key was generated with the following command: `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -sha256 -subj '/CN=localhost'`