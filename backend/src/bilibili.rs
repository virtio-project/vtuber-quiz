use std::fmt;

use serde::{Serialize, Deserializer, Deserialize};
use reqwest::get;


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Response<T> {
    code: i32,
    message: String,
    ttl: i32,
    data: Option<T>
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Bilbili(#[from] BilbiliError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error)
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

impl <T> Response<T> {
    fn success(&self) -> bool {
        self.code == 0
    }

    fn into_error(self) -> BilbiliError {
        self.into()
    }
}

impl AccountInfo {
    async fn get_by_id(uid: u64) -> Result<Self, Error> {
        let url = format!("https://api.bilibili.com/x/space/acc/info?mid={}&jsonp=jsonp", uid);
        let response: Response<Self> = get(url).await?.json().await?;
        if response.success() {
            Ok(response.data.unwrap())
        } else {
            Err(response.into_error())?
        }
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

impl <T> From<Response<T>> for BilbiliError {
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
        let err = AccountInfo::get_by_id(1000000000).await.unwrap_err().unwrap_bilibili();
        assert_eq!(err.code, -404);
    }
}