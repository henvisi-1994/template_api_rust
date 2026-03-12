use crate::requests::request_auth_register::ProfilePayload;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct UserAuthRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role_id: Option<i32>,
    pub status: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct AuthMeRow {
    pub user_id: Uuid,
    pub email: String,
    pub profile_id: Option<i64>,
    pub role_id: Option<i32>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub profile_pic_url: Option<String>,
    pub municipio_id: Option<i64>,
    pub provincia_id: Option<i64>,
    pub provider_id: Option<i64>,
    pub status: Option<String>,
}

pub struct RegisterAccountTxnData {
    pub user_id: Uuid,
    pub tenant_id: i64,
    pub profile_id: i64,
    pub provider_id: Option<i64>,
    pub profile_type: &'static str,
    pub profile_pic_url: Option<String>,
}

pub struct CreateProfileArgs<'a> {
    pub profile: &'a ProfilePayload,
    pub role_id: i64,
    pub user_id: Uuid,
    pub tenant_id: i64,
    pub location_id: i64,
    pub profile_type: String,
    pub profile_pic_url: Option<String>,
}
