use vercel_runtime::{run, Body, Request, Response, StatusCode};
use friendbank::models::UserData;
use friendbank::services::user_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(|req: Request| async move {
        let body_bytes = req.body().to_vec();
        let user_data: UserData = match serde_json::from_slice(&body_bytes) {
            Ok(ud) => ud,
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::Text("{\"error\":\"JSON inválido\"}".to_string()))
                    .unwrap());
            }
        };

        match user_service::login(&user_data).await {
            Ok(login_response) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::Text(serde_json::to_string(&login_response).unwrap()))
                .unwrap()),
            Err(err) => Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::Text(format!("{{\"error\":\"{}\"}}", err)))
                .unwrap()),
        }
    })
    .await
}

