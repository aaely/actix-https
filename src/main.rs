use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_files::Files;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load SSL keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("s.key", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("/usr/local/share/ca-certificates/s.crt").unwrap();

    // Start HTTPS server
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind_openssl("192.168.4.160:8080", builder)?
    .run()
    .await
}