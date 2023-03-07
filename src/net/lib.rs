use axum::{ BoxError, extract::Host, handler::HandlerWithoutStateExt, http::{ StatusCode, Uri }, response::Redirect, };
use std::net::SocketAddr;

use crate::net::models::ports::Ports;

pub async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http));

    axum::Server::bind(&addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
} 
