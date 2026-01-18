use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct RefreshTokenEntity {
    pub id: Uuid,
    pub user_id: i32,
    pub expires_at: NaiveDateTime,
    pub revoked: bool,
    pub created_at: NaiveDateTime,
}

impl RefreshTokenEntity {
    pub fn is_invalid(&self) -> bool {
        let now = chrono::Utc::now().naive_utc();
        self.revoked || self.expires_at < now
    }
}

