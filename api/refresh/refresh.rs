use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};
use friendbank::services::auth_service::refresh;
use friendbank::models::dto::auth_dto::{
    RefreshTokenRequest,
};
use friendbank::errors::http_errors::map_auth_error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if req.method() != "POST" {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Text("Only POST".to_string()))?);
    }

    let body_bytes = req.body().to_vec();

    let refresh_token_request: RefreshTokenRequest =
        match serde_json::from_slice(&body_bytes) {
            Ok(req) => req,
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::Text("{\"error\":\"JSON invalido\"}".to_string()))?);
            }
        };

    match refresh(refresh_token_request).await {
        Ok(resp) => Ok(
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::Text(serde_json::to_string(&resp).unwrap()))?
        ),

Err(err) => {
    let status = map_auth_error(&err);

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::Text(format!("{{\"error\":\"{:?}\"}}", err)))?)
}

    }
}


