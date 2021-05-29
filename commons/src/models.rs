use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequest {
    #[serde(default = "default_false")]
    pub private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResponse {
    pub code: String,
    pub templates: Vec<String>
}

#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "user_role", rename_all = "lowercase"))]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Normal,
    Vtuber,
}

#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "question_type", rename_all = "snake_case"))]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    TrueFalse,
    MultiChoice,
    MultiAnswer,
}

#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "vote_action", rename_all = "snake_case"))]
#[derive(Copy, Clone, Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoteAction {
    UpVote,
    DownVote,
    FlagOutdated,
    FlagIncorrect,
}

#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "audience", rename_all = "lowercase"))]
#[derive(Copy, Clone, Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    Vtuber,
    Fan,
    Passenger,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    #[cfg(feature = "backend")]
    pub password: String,
    #[serde(skip_serializing)]
    #[cfg(feature = "backend")]
    pub challenge: Option<String>,
    pub role: UserRole,
    pub blocked: bool,
    pub reputation: i32,
    #[serde(with = "ts_milliseconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated: DateTime<Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub creator: i32,
    pub description: String,
    pub choices: Vec<String>,
    pub answer: Vec<i32>,
    #[cfg_attr(feature = "backend", sqlx(rename = "type"))]
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub audiences: Vec<Audience>,
    pub draft: bool,
    pub deleted: bool,
    #[serde(with = "ts_milliseconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated: DateTime<Utc>,
}

impl ChallengeResponse {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            // todo: configurable
            templates: vec![
                "我正在使用vtuber粉丝力测试，点下方的链接看看我创建的题目吧\nhttps://quiz.virtio.com.cn/v/{}".to_string(),
                "我正在使用vtuber粉丝力测试，点下方的链接测试一下你的粉丝力吧\nhttps://quiz.virtio.com.cn/v/{}".to_string(),
            ]
        }
    }
}

const fn default_false() -> bool { false }