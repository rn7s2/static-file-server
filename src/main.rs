use actix_files as fs;
use actix_web::{App, HttpServer};
use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// e.g. "/static/", default "/"
    #[arg(long, short)]
    endpoint: Option<String>,

    /// e.g. ".", default "."
    #[arg(long, short)]
    dir: Option<String>,

    /// e.g. "127.0.0.1", default "0.0.0.0"
    #[arg(long, short)]
    ip: Option<String>,

    /// e.g. 8080, default 8080
    #[arg(long, short)]
    port: Option<u16>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let (endpoint, dir, ip, port) = (
        match args.endpoint {
            Some(v) => {
                if !v.ends_with("/") {
                    v + "/"
                } else {
                    v
                }
            }
            None => "/".to_owned(),
        },
        match args.dir {
            Some(v) => v,
            None => ".".to_owned(),
        },
        match args.ip {
            Some(v) => v,
            None => "0.0.0.0".to_owned(),
        },
        match args.port {
            Some(v) => v,
            None => 8080,
        },
    );

    println!(
        "Static file server serving on: http://{}:{}{}",
        ip, port, endpoint
    );

    HttpServer::new(move || {
        App::new().service(fs::Files::new(&endpoint, &dir).show_files_listing())
    })
    .bind((ip, port))?
    .run()
    .await
}
