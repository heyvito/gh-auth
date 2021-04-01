use crate::config::Config;
use hyper::Method;
use url::form_urlencoded;
use std::collections::HashMap;
use std::sync::Arc;
use hyper::{Request, Body};
use std::borrow::Cow;
use serde_json::Value;
use hyper::body::HttpBody;
use hyper_tls::HttpsConnector;
use hyper::client::Client;

pub(crate) async fn exchange_code(cfg: &Arc<Config>, code: &str) -> Option<String> {
    let https = HttpsConnector::new();
    let http_client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::builder()
        .method(Method::POST)
        .uri("https://github.com/login/oauth/access_token")
        .body(Body::from(form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", cfg.github_key.as_str())
            .append_pair("client_secret", cfg.github_secret.as_str())
            .append_pair("code", code)
            .finish()))
        .expect("request builder");
    let mut response = match http_client.request(req).await {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            return None;
        }
    };
    let response = match response.data().await {
        None => return None,
        Some(v) => v,
    }.ok()?;
    let response = url::form_urlencoded::parse(response.as_ref()).collect::<HashMap<_, _>>();
    let token_key = Cow::Borrowed("access_token");
    if !response.contains_key(&token_key) {
        return None;
    }

    Some(response.get(&token_key).unwrap().to_string())
}

pub(crate) async fn get_username(token: String) -> Option<String> {
    let https = HttpsConnector::new();
    let http_client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://api.github.com/user")
        .header("User-Agent", "gh-auth/1.0 (https://github.com/heyvito/gh-auth)")
        .header("Authorization", format!("token {}", token))
        .body(Body::empty())
        .expect("request builder");
    let response = match http_client.request(req).await {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            return None;
        }
    };
    let body_bytes = match hyper::body::to_bytes(response.into_body()).await {
        Err(err) => {
            eprintln!("{}", err);
            return None;
        },
        Ok(v) => v,
    };
    let json: Value = match serde_json::from_slice(body_bytes.as_ref()) {
        Err(e) => {
            println!("{:?}", String::from_utf8(body_bytes.as_ref().to_vec()));
            eprintln!("{}", e);
            return None;
        }
        Ok(v) => v
    };
    Some(json["login"].as_str().unwrap().to_string())
}
