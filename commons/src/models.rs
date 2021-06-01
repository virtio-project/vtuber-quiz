use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
pub struct QuestionCreationRequest {
    pub content: QuestionContent,
    pub audiences: HashSet<Audience>,
    pub draft: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionContent {
    TrueFalse(TrueFalseQuestion),
    MultiChoice(MultiChoiceQuestion),
    MultiAnswer(MultiAnswerQuestion),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrueFalseQuestion {
    pub description: String,
    pub is_true: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiChoiceQuestion {
    pub description: String,
    pub choices: Vec<String>,
    pub answer: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiAnswerQuestion {
    pub description: String,
    pub choices: Vec<String>,
    pub answer: Vec<usize>,
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
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
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
    pub audiences: Vec<String>,
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
    pub fn audiences(&self) -> Vec<String> {
        self.audiences
            .iter()
            .map(|audience| serde_json::to_string(audience).unwrap())
            .collect()
    }
}

impl QuestionContent {
    pub fn description(&self) -> &str {
        use QuestionContent::*;

        match self {
            TrueFalse(q) => q.description.as_str(),
            MultiChoice(q) => q.description.as_str(),
            MultiAnswer(q) => q.description.as_str(),
        }
    }

    pub fn choices(&self) -> Vec<String> {
        use QuestionContent::*;

        match self {
            TrueFalse(_) => vec!["T".to_string(), "F".to_string()],
            MultiChoice(q) => q.choices.clone(),
            MultiAnswer(q) => q.choices.clone(),
        }
    }

    pub fn answer(&self) -> Vec<i32> {
        use QuestionContent::*;

        match self {
            TrueFalse(q) => if q.is_true { vec![0] } else { vec![1] },
            MultiChoice(q) => vec![q.answer as i32],
            MultiAnswer(q) => q.answer.iter().map(|u| *u as i32).collect(),
        }
    }

    pub fn ty(&self) -> QuestionType {
        use QuestionContent::*;

        match self {
            TrueFalse(_) => QuestionType::TrueFalse,
            MultiChoice(_) => QuestionType::MultiChoice,
            MultiAnswer(_) => QuestionType::MultiAnswer,
        }
    }

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

impl Question {
    pub fn is_valid(&self) -> bool {
        use QuestionType::*;

        let choices_len = self.choices.len();
        let answer_len = self.answer.len();
        match self.question_type {
            TrueFalse => {
                if self.choices != vec!["T".to_string(), "F".to_string()] {
                    return false;
                }
                if answer_len != 1 {
                    return false;
                }
            }
            MultiChoice => {
                if answer_len != 1 {
                    return false;
                }
                if (self.answer[0] as usize) >= choices_len {
                    return false;
                }
            }
            MultiAnswer => {
                if answer_len > choices_len {
                    return false;
                }
                if !self.answer.iter().all(|i| (*i as usize) < choices_len) {
                    return false;
                }
            }
        }
        true
    }

    /// panic: when the question is not valid or not a true-false question
    pub fn unwrap_true_false(&self) -> TrueFalseQuestion {
        assert_eq!(self.question_type, QuestionType::TrueFalse);
        assert!(self.is_valid());

        TrueFalseQuestion {
            description: self.description.clone(),
            is_true: self.answer[0] == 0
        }
    }

    /// panic: when the question is not valid or not a multi-choice question
    pub fn unwrap_multi_choice(&self) -> MultiChoiceQuestion {
        assert_eq!(self.question_type, QuestionType::MultiChoice);
        assert!(self.is_valid());

        MultiChoiceQuestion {
            description: self.description.clone(),
            choices: self.choices.clone(),
            answer: self.answer[0] as usize
        }
    }

    /// panic: when the question is not valid or not a multi-answer question
    pub fn unwrap_multi_answer(&self) -> MultiAnswerQuestion {
        assert_eq!(self.question_type, QuestionType::MultiChoice);
        assert!(self.is_valid());

        MultiAnswerQuestion {
            description: self.description.clone(),
            choices: self.choices.clone(),
            answer: self.answer.iter().map(|i| *i as usize).collect()
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
        let question: QuestionContent = serde_json::from_str(tf_json).unwrap();
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
        let question: QuestionContent = serde_json::from_str(mc_json).unwrap();
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
        let question: QuestionContent = serde_json::from_str(ma_json).unwrap();
        let multi_answer = question.unwrap_multi_answer();
        assert_eq!(multi_answer.description.as_str(), "select all words describe color");
        assert_eq!(multi_answer.choices.len(), 4);
        assert_eq!(multi_answer.answer, vec![0, 1, 2]);
    }
}