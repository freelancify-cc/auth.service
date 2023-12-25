use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub user_role_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserProfile {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub second_name: String,
    pub date_of_birth: String,
    pub username: String,
    pub profile_picture_url: String,
    pub user_id: uuid::Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserContactInformation {
    pub id: uuid::Uuid,
    pub contact: String,
    pub user_profile_id: uuid::Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserBankingInformation {
    pub id: uuid::Uuid,
    pub contact: String,
    pub user_profile_id: uuid::Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FilteredUserModel {
    pub id: uuid::Uuid,
    pub email: String,
    pub role: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FilteredUserProfileModel {
    pub username: String,
    pub first_name: String,
    pub second_name: String,
    pub profile_picture_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInformationModel {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub second_name: String,
    pub profile_picture_url: String,
    pub date_of_birth: String,
    pub role_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSkills {
    pub user: String,
    pub skills: Vec<String>,
}

pub fn filter_user_record(user: &User) -> FilteredUserModel {
    FilteredUserModel {
        id: user.id,
        email: user.email.to_owned(),
        role: user.user_role_id.to_owned(),
    }
}

pub fn filter_userprofile_record(profile: &UserProfile) -> FilteredUserProfileModel {
    FilteredUserProfileModel {
        username: profile.username.to_lowercase().to_owned(),
        first_name: profile.first_name.to_lowercase().to_owned(),
        second_name: profile.second_name.to_lowercase().to_owned(),
        profile_picture_url: profile.profile_picture_url.to_owned(),
    }
}

pub fn filter_userinformation_record(
    user: &User,
    profile: &UserProfile,
    role: String,
) -> UserInformationModel {
    UserInformationModel {
        id: user.id,
        email: user.email.to_owned(),
        username: profile.username.to_owned(),
        first_name: profile.first_name.to_owned(),
        second_name: profile.second_name.to_owned(),
        profile_picture_url: profile.profile_picture_url.to_owned(),
        date_of_birth: profile.date_of_birth.to_owned(),
        role_name: role,
    }
}
