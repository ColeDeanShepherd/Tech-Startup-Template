mod users;
use users::{get_user, create_user, delete_user};

mod messages;
use messages::{get_messages, send_message, delete_message};

mod videos;
use videos::{upload_video, delete_video};

mod comments;
use comments::{post_comment};

use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

const PRIVATE_KEY_FILENAME: &str = "key.pem";
const CERTIFICATE_CHAIN_FILENAME: &str = "cert.pem";

const REST_API_URL_AUTHORITY: &str = "127.0.0.1:8080";
const REST_API_URL: &str = "https://127.0.0.1:8080";

fn create_ssl_builder() -> SslAcceptorBuilder {
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(PRIVATE_KEY_FILENAME, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(CERTIFICATE_CHAIN_FILENAME).unwrap();

    builder
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting REST API at {}", REST_API_URL);
    
    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(create_user)
            .service(delete_user)
            
            .service(get_messages)
            .service(send_message)
            .service(delete_message)

            .service(upload_video)
            .service(delete_video)

            .service(post_comment)
    })
    .bind_openssl(REST_API_URL_AUTHORITY, create_ssl_builder())?
    .run()
    .await
}
