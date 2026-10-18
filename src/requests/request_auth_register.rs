use chrono::NaiveDate;
use serde::Deserialize;

/// Minimal register payload for the template.
#[derive(Deserialize)]
pub struct AuthRegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct LocationPayload {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub location_source: Option<String>,
    pub region_text: Option<String>,
    pub region_code: Option<String>,
    pub address_text: Option<String>,
}

#[derive(Deserialize)]
pub struct ProfilePayload {
    pub name: String,
    pub phone: String,
    pub municipio_id: Option<i64>,
    pub provincia_id: Option<i64>,
    pub location: LocationPayload,
    pub profile_pic_url: Option<String>,
    pub role_id: Option<i64>,
    #[serde(alias = "type")]
    pub profile_type: Option<String>,
    pub person: Option<ProfilePersonPayload>,
    pub company: Option<ProfileCompanyPayload>,
    pub tenant_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ProfilePersonPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub document_type: Option<String>,
    pub document_number: Option<String>,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct ProfileCompanyPayload {
    pub legal_name: Option<String>,
    pub trade_name: Option<String>,
    pub tax_id: Option<String>,
    pub contact_person_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ProviderPayload {
    pub phone: Option<String>,
    pub description: String,
    pub speciality: String,
    pub availability: String,
    pub address: Option<String>,
    pub is_premium: Option<bool>,
    pub plan_id: Option<i64>,
    pub position: Option<i32>,
}

#[derive(Deserialize)]
pub struct AuthRegisterProviderRequest {
    pub email: String,
    pub password: String,
    pub profile: ProfilePayload,
    pub provider: ProviderPayload,
}

#[derive(Deserialize)]
pub struct AuthRegisterProfileRequest {
    pub email: String,
    pub password: String,
    pub profile: ProfilePayload,
}
