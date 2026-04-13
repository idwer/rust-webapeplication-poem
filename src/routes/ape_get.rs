use poem::handler;
use poem::IntoResponse;
use poem::Response;
use poem::Result;
use poem::http::StatusCode;

#[handler]
pub async fn ape_get() -> Result<impl IntoResponse> {
    let body = concat!("Using the endpoint, height and wingspan are in centimeter:<br>curl --header ", r#"'Content-Type: application/json' http://127.0.0.1:8080/ape -X POST -d '{"height": 200, "wingspan": 200}'"#);

    Ok(Response::builder()
      .status(StatusCode::OK)
      .content_type("text/html; charset=utf-8")
      .body(body))
}

