use hyper::{Request, Body, Response, Method, StatusCode};
use crate::{simple_html, github, cookie};
use crate::config::Config;
use std::sync::Arc;
use url::Url;
use std::collections::HashMap;
use std::borrow::Cow;
use crate::cookie::decode;
use std::path::Path;
use hyper_staticfile::Static;

fn url_for(req: &Request<Body>, path: &str) -> Option<String> {
    let host = match req.headers().get("Host") {
        None => return None,
        Some(h) => h.to_str().unwrap()
    };
    let scheme = req.uri().scheme_str().unwrap_or("http");
    Some(format!("{}://{}{}", scheme, host, path))
}

fn bad_request(message: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/html")
        .body(Body::from(simple_html::make_page("Bad Request", message)))
        .unwrap()
}

fn forbidden(message: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "text/html")
        .body(Body::from(simple_html::make_page("Access Denied", message)))
        .unwrap()
}

fn server_error(message: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "text/html")
        .body(Body::from(simple_html::make_page("Error", message)))
        .unwrap()
}

fn redirect_to_login(req: &Request<Body>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header("Location", url_for(req, "/__gh_auth/login").unwrap())
        .body(Body::empty())
        .unwrap()
}

pub async fn handle(root: Arc<String>, cfg: Arc<Config>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/__gh_auth/login") => {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Body::from(simple_html::make_login()))
                .unwrap())
        },
        (&Method::GET, "/__gh_auth/begin") => {
            let url = format!("https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&allow_signup=false",
                              cfg.github_key, urlencoding::encode(&url_for(&req, "/__gh_auth/callback").unwrap()));
            Ok(Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header("Location", url)
                .body(Body::empty())
                .unwrap())
        }

        (&Method::GET, "/__gh_auth/callback") => {
            let url;
            let query = if let Some(query) = req.uri().query() {
                url = Url::parse(&format!("https://example.com/?{}", query.to_string())).unwrap();
                url.query_pairs().collect::<HashMap<_, _>>().to_owned()
            } else {
                return Ok(bad_request("Invalid authorization code"));
            };

            let code_param = Cow::Borrowed("code");
            if !query.contains_key(&code_param) {
                return Ok(bad_request("Invalid authorization code"));
            }

            let code = query.get(&code_param).unwrap().to_string();
            let token = match github::exchange_code(&cfg, &code).await {
                None => return Ok(bad_request("Invalid response from upstream authentication service")),
                Some(v) => v
            };

            let login = match github::get_username(token).await {
                None => return Ok(bad_request("Invalid response from upstream authentication service")),
                Some(v) => v
            };

            if !cfg.allowed_users.contains(&login) {
                return Ok(forbidden("Access denied."));
            }

            let cookie = cookie::create(&cfg, login.to_string());
            if cookie == None {
                return Ok(server_error("Internal error."));
            }



            Ok(Response::builder()
                .header("Set-Cookie", format!("__gh_auth_session={}; HttpOnly; Path=/", cookie.unwrap()))
                .header("Location", url_for(&req, "").unwrap())
                .status(StatusCode::TEMPORARY_REDIRECT)
                .body(Body::empty())
                .unwrap())
        }

        _ => {
            let raw_cookie = req.headers().get("Cookie");
            if raw_cookie == None {
                return Ok(redirect_to_login(&req));
            }
            let cookies = raw_cookie.unwrap().to_str().unwrap()
                .split(";")
                .map(|c| c.trim())
                .map(|c| {
                    let vals = c.split("=").collect::<Vec<&str>>();
                    (vals[0], vals[1])
                })
                .collect::<HashMap<_, _>>();
            if !cookies.contains_key("__gh_auth_session") {
                return Ok(redirect_to_login(&req));
            }
            let name = match decode(&cfg, cookies["__gh_auth_session"].to_string()) {
                None => return Ok(redirect_to_login(&req)),
                Some(n) => n
            };

            if !cfg.allowed_users.contains(&name) {
                return Ok(Response::builder()
                    .status(StatusCode::TEMPORARY_REDIRECT)
                    .header("Set-Cookie", "__gh_session_id=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; Path=/")
                    .header("Location", url_for(&req, "").unwrap())
                    .body(Body::empty())
                    .unwrap());
            }

            // Serve files!
            match Static::new(Path::new(&*root)).serve(req).await {
                Ok(v) => Ok(v),
                Err(e) => {
                    eprintln!("{}", e);
                    Ok(server_error("An error occurred serving files"))
                }
            }
        }
    }
}
