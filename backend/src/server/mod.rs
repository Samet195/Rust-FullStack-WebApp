use std::sync::mpsc;

use actix_server::ServerHandle;
use actix_web::{
    http::Method,
    middleware::{Logger, NormalizePath},
    web::Path,
    *,
};
use common::{types::ErrorJson, Config};
use mime_guess::from_path as mime;
use rust_i18n::t;

mod api;
mod file;
mod tls;

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(format!(
        include!("../../assets/index.html"),
        t!("title")
    ))
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    let path = "favicon.ico";

    match file::get(path).await {
        Some(content) => HttpResponse::Ok()
            .content_type(mime(path).first_or_octet_stream().as_ref())
            .body(content),

        None => HttpResponse::NotFound().json(ErrorJson {
            message: "404 Not Found".into(),
        }),
    }
}

#[get("/{path:.*}")]
async fn scripts(path: Path<String>) -> impl Responder {
    let path = path.to_string();

    match file::get(&path).await {
        Some(content) => HttpResponse::Ok()
            .content_type(mime(path).first_or_octet_stream().as_ref())
            .body(content),

        None => HttpResponse::NotFound().json(ErrorJson {
            message: "404 Not Found".into(),
        }),
    }
}

#[get("/static/{path:.*}")]
async fn assets(path: Path<String>) -> impl Responder {
    let path = path.to_string();

    match file::get(&path).await {
        Some(content) => HttpResponse::Ok()
            .content_type(mime(path).first_or_octet_stream().as_ref())
            .body(content),

        None => HttpResponse::NotFound().json(ErrorJson {
            message: "404 Not Found".into(),
        }),
    }
}

pub async fn run(
    tx_port: mpsc::Sender<u16>,
    tx_handle: Option<mpsc::Sender<ServerHandle>>,
) -> std::io::Result<()> {
    let config = Config::load_from_embeded().unwrap_or_default();

    let server = HttpServer::new(move || {
        App::new()
            .default_service(web::to(index).method(Method::GET))
            .service(assets)
            .service(web::scope("/api").service(api::test).service(api::test_d))
            .wrap(Logger::new("%s %r"))
            .wrap(NormalizePath::trim())
    });

    let addr = if cfg!(feature = "internal_port") {
        (config.addr.internal, 0)
    } else {
        (config.addr.public, {
            if cfg!(feature = "tls") {
                config.port.https
            } else {
                config.port.http
            }
        })
    };

    #[cfg(not(feature = "tls"))]
    let server = server.bind(addr)?;

    #[cfg(feature = "tls")]
    let server = server.bind_rustls_0_22(addr, tls::load_config())?;

    let port = server.addrs().first().unwrap().port();
    tx_port.send(port).unwrap();

    let server = server.run();

    if let Some(tx) = tx_handle {
        tx.send(server.handle()).unwrap();
    }

    server.await
}
