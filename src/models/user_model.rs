use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use crate::config::tools::deserialize_boolean;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub login: String,
    pub password: String,
    #[serde(rename = "fistName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub email: String,
    #[serde(rename = "institutionId")]
    pub institution_id: Option<String>,
    #[serde(deserialize_with = "deserialize_boolean")]
    pub active: bool,
    pub role: i32,
}

impl UserModel {
    pub fn get_passw_only(&self) -> String {
        if self.password.len() >= 128 {
            self.password[..128].to_string()
        } else {
            self.password.clone()
        }
    }
    pub fn get_salt(&self) -> String {
        if self.password.len() > 128 {
            self.password[128..].to_string()
        } else {
            "".to_string()
        }
    }
}
