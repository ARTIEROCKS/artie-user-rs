use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub login: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub institution_id: Option<String>,
    pub active: bool,
    pub role: i32,
}
