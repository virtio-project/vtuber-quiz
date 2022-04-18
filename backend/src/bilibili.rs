use std::fmt;

use reqwest::get;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Response<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Bilbili(#[from] BilbiliError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("got unexpected response")]
    UnexpectedResponse,
}

#[derive(Clone, Debug, Serialize, thiserror::Error)]
struct BilbiliError {
    code: i32,
    message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename(deserialize = "mid"))]
    pub uid: u64,
    pub name: String,
    pub sex: String,
    pub face: String,
    pub level: u8,
    #[serde(deserialize_with = "from_u8")]
    pub silence: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct PostInfo {
    pub rid: u64,
    pub sender: AccountInfo,
    pub content: String,
}

impl<T> Response<T> {
    fn success(&self) -> bool {
        self.code == 0
    }

    fn into_error(self) -> BilbiliError {
        self.into()
    }
}

impl AccountInfo {
    async fn get_by_id(uid: u64) -> Result<Self, Error> {
        let url = format!(
            "https://api.bilibili.com/x/space/acc/info?mid={}&jsonp=jsonp",
            uid
        );
        let response: Response<Self> = get(url).await?.json().await?;
        if response.success() {
            Ok(response.data.unwrap())
        } else {
            Err(response.into_error().into())
        }
    }
}

impl PostInfo {
    async fn get_by_id(rid: u64) -> Result<Self, Error> {
        let url = format!("https://api.vc.bilibili.com/dynamic_svr/v1/dynamic_svr/get_dynamic_detail?dynamic_id={}", rid);
        let response: Response<serde_json::Value> = get(url).await?.json().await?;
        if !response.success() {
            return Err(response.into_error().into());
        }
        let data = response.data.unwrap();
        let inner = data.get("card").ok_or(Error::UnexpectedResponse)?;
        let card = inner
            .get("card")
            .and_then(|v| v.as_str())
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
            .ok_or(Error::UnexpectedResponse)?;
        let uid = card
            .get("user")
            .and_then(|u| u.get("uid"))
            .and_then(|u| u.as_u64())
            .ok_or(Error::UnexpectedResponse)?;
        let sender = AccountInfo::get_by_id(uid).await?;
        let item = card.get("item").ok_or(Error::UnexpectedResponse)?;
        let rid = item
            .get("rp_id")
            .and_then(|u| u.as_u64())
            .ok_or(Error::UnexpectedResponse)?;
        let content = item
            .get("content")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or(Error::UnexpectedResponse)?;
        Ok(Self {
            sender,
            rid,
            content,
        })
    }
}

impl Error {
    pub fn unwrap_bilibili(self) -> BilbiliError {
        if let Self::Bilbili(b) = self {
            b
        } else {
            panic!("Error variant mismatch")
        }
    }
}

impl<T> From<Response<T>> for BilbiliError {
    fn from(r: Response<T>) -> Self {
        Self {
            code: r.code,
            message: r.message,
        }
    }
}

impl fmt::Display for BilbiliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bilibili error({}): {}", self.code, self.message)
    }
}

fn from_u8<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u8 = Deserialize::deserialize(deserializer)?;
    if s == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_account_info() {
        let info = AccountInfo::get_by_id(546195).await.unwrap();
        assert_eq!(info.uid, 546195);
        assert_eq!(info.name.as_str(), "老番茄");
        assert_eq!(info.level, 6);
    }

    #[tokio::test]
    async fn test_get_account_info_err() {
        let err = AccountInfo::get_by_id(1000000000)
            .await
            .unwrap_err()
            .unwrap_bilibili();
        assert_eq!(err.code, -404);
    }

    #[tokio::test]
    async fn test_get_post_info() {
        let info = PostInfo::get_by_id(85387458835565545).await.unwrap();
        assert_eq!(info.rid, 85387458833966060);
        assert_eq!(info.sender.uid, 2);
        assert_eq!(info.sender.name, "碧诗");
        assert!(!info.content.is_empty());
    }
}
