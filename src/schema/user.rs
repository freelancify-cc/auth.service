use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserSchema {
    pub email: String,
    pub password: String,
}

pub struct CreateUserProfileSchema {
    pub username: String,
    pub first_name: String,
    pub second_name: String,
    pub date_of_birth: String,
    pub profile_picture_url: String 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserCredentialsSchema {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUserSchema {
    pub username: Option<String>,
    pub password: Option<String>,
}
