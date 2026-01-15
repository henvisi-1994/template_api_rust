use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct CityResource {
    pub name: Option<String>,
    pub region: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
