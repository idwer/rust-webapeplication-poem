use poem::handler;
use poem::Response;
use poem::http::StatusCode;

#[handler]
pub async fn handle_404() -> Response {
    Response::builder()
    .status(StatusCode::NOT_FOUND)
    .content_type("text/html; charset=utf-8")
    .body(StatusCode::NOT_FOUND.to_string())
}

