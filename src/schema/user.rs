use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserSchema {
    pub email: String,
    pub password: String,
    pub role: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserProfileSchema {
    pub id: uuid::Uuid,
    pub username: String,
    pub first_name: String,
    pub second_name: String,
    pub date_of_birth: String,
    pub profile_picture_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserCredentialsSchema {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FreelancerSkillsSchema {
    pub skills: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FreelancerExperienceSchema {
    pub experience: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FreelancerEducationSchema {
    pub education: Vec<String>,
}
