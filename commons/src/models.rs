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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionCreationRequest {
    TrueFalse(TrueFalseQuestion),
    MultiChoice(MultiChoiceQuestion),
    MultiAnswer(MultiAnswerQuestion),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrueFalseQuestion {
    description: String,
    is_true: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiChoiceQuestion {
    description: String,
    choices: Vec<String>,
    answer: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiAnswerQuestion {
    description: String,
    choices: Vec<String>,
    answer: Vec<usize>,
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
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoteAction {
    UpVote,
    DownVote,
    FlagOutdated,
    FlagIncorrect,
}

#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "audience", rename_all = "lowercase"))]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
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

impl QuestionCreationRequest {
    pub fn unwrap_true_false(self) -> TrueFalseQuestion {
        if let Self::TrueFalse(q) = self {
            q
        } else {
            panic!("variant mismatch")
        }
    }

    pub fn unwrap_multi_choice(self) -> MultiChoiceQuestion {
        if let Self::MultiChoice(q) = self {
            q
        } else {
            panic!("variant mismatch")
        }
    }

    pub fn unwrap_multi_answer(self) -> MultiAnswerQuestion {
        if let Self::MultiAnswer(q) = self {
            q
        } else {
            panic!("variant mismatch")
        }
    }
}

const fn default_false() -> bool { false }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_false() {
        let tf_json: &str = r#"{
  "description": "1+1=3",
  "is_true": false
}"#;
        let question: QuestionCreationRequest = serde_json::from_str(tf_json).unwrap();
        let true_false = question.unwrap_true_false();
        assert_eq!(true_false.description.as_str(), "1+1=3");
        assert_eq!(true_false.is_true, false);
    }

    #[test]
    fn test_multi_choice() {
        let mc_json: &str = r#"{
  "description": "select a prime number from those numbers",
  "choices": ["1", "2", "4", "8"],
  "answer": 1
}"#;
        let question: QuestionCreationRequest = serde_json::from_str(mc_json).unwrap();
        let multi_choice = question.unwrap_multi_choice();
        assert_eq!(multi_choice.description.as_str(), "select a prime number from those numbers");
        assert_eq!(multi_choice.choices.len(), 4);
        assert_eq!(multi_choice.answer, 1);
    }

    #[test]
    fn test_multi_answer() {
        let ma_json: &str = r#"{
  "description": "select all words describe color",
  "choices": ["black", "red", "green", "happy"],
  "answer": [0, 1, 2]
}"#;
        let question: QuestionCreationRequest = serde_json::from_str(ma_json).unwrap();
        let multi_answer = question.unwrap_multi_answer();
        assert_eq!(multi_answer.description.as_str(), "select all words describe color");
        assert_eq!(multi_answer.choices.len(), 4);
        assert_eq!(multi_answer.answer, vec![0, 1, 2]);
    }
}