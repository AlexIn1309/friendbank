use vercel_runtime::Request;

use crate::auth::guard::{authenticate_bearer, require_accountant};
use crate::auth::context::AuthContext;
use crate::errors::user_error::UserError;

pub fn auth_accountant(req: &Request) -> Result<AuthContext, UserError> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());

    let ctx = authenticate_bearer(auth_header)?;
    require_accountant(&ctx)?;

    Ok(ctx)
}
