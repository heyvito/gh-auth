use std::process::exit;
use clap::{App, Arg};
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Server};
use std::sync::Arc;
use std::path::Path;

mod cookie;
mod config;
mod handler;
mod simple_html;
mod github;

#[tokio::main]
async fn main() {
    let matches = App::new("GHAuth")
        .version("1.0")
        .author("Victor Gama <hey@vito.io>")
        .about("Provides an HTTP server that requires authentication using GitHub")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("IP")
            .help("Sets the host to bind to")
            .default_value("0.0.0.0")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Sets the port to bind to")
            .default_value("8000")
            .takes_value(true))
        .arg(Arg::with_name("root")
            .short("r")
            .long("root")
            .help("Sets the root directory to serve files from")
            .value_name("DIR")
            .required(true)
            .takes_value(true))
        .get_matches();

    let config = match config::read_config() {
        Err(e) => {
            eprintln!("Error: {}", e.message);
            exit(1);
        }
        Ok(v) => v
    };

    let raw_root = matches.value_of("root");
    let root = raw_root.unwrap().to_owned();
    if !Path::new(&root).exists() {
       eprintln!("ERROR: Root path {} does not exist!", root);
        exit(1);
    }

    let raw_bind = format!("{}:{}",
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let addr: SocketAddr = match raw_bind.parse() {
        Err(e) => {
            eprint!("ERROR: Invalid host/port combination: {}", e);
            exit(1)
        }
        Ok(v) => v
    };
    let cfg_arc = Arc::new(config);
    let root_arc = Arc::new(root);
    let service = make_service_fn(move |_| {
        let service_arc = cfg_arc.clone();
        let root = root_arc.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handler::handle(root.clone(), service_arc.clone(), req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
