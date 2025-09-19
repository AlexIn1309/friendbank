use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};
use friendbank::services::user_service;
use friendbank::models::UserData;

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
    let user_data: UserData = match serde_json::from_slice(&body_bytes) {
        Ok(ud) => ud,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Text("{\"error\":\"JSON inválido\"}".to_string()))?);
        }
    };

    match user_service::login(&user_data).await {
        Ok(login_response) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::Text(serde_json::to_string(&login_response).unwrap()))?),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::Text(format!("{{\"error\":\"{}\"}}", err)))?),
    }
}

