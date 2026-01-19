
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};

use friendbank::services::user_service;
use friendbank::models::dto::auth_dto::RegisterRequest;

use friendbank::auth::guard::{authenticate_bearer, require_accountant};
use friendbank::errors::http_errors::map_auth_error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if req.method() != "POST" {
        return Ok(
            Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::Text("Only POST".to_string()))?
        );
    }

    // ?? AUTH
    let ctx = match authenticate_bearer(
        req.headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok()),
    ) {
        Ok(ctx) => ctx,
        Err(err) => {
            return Ok(
                Response::builder()
                    .status(map_auth_error(&err))
                    .body(Body::Text(format!("{{\"error\":\"{:?}\"}}", err)))?
            );
        }
    };

    // ?? ROLE
    if let Err(err) = require_accountant(&ctx) {
        return Ok(
            Response::builder()
                .status(map_auth_error(&err))
                .body(Body::Text(format!("{{\"error\":\"{:?}\"}}", err)))?
        );
    }

    // ?? BODY
    let body_bytes = req.body().to_vec();
    let register_request: RegisterRequest = match serde_json::from_slice(&body_bytes) {
        Ok(req) => req,
        Err(_) => {
            return Ok(
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::Text("{\"error\":\"JSON invalido\"}".to_string()))?
            );
        }
    };

    // ?? SERVICE
    match user_service::register(&register_request).await {
        Ok(user) => Ok(
            Response::builder()
                .status(StatusCode::CREATED)
                .header("Content-Type", "application/json")
                .body(Body::Text(serde_json::to_string(&user).unwrap()))?
        ),
        Err(err) => Ok(
            Response::builder()
                .status(map_auth_error(&err))
                .body(Body::Text(format!("{{\"error\":\"{:?}\"}}", err)))?
        ),
    }
}

