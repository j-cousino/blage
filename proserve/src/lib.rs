use std::net::SocketAddr;

use axum::{extract::ConnectInfo, response::IntoResponse, routing::get, Router};
use axum_extra::TypedHeader;

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(handler))
}

async fn handler(
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> String {
    let mut user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        "Unknown browser".to_string()
    };

    let address = format!("\n{}", addr.to_string());
    user_agent.push_str( &address);

    user_agent

}
