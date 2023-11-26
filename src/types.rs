use serde::{Deserialize, Serialize};
use time::{format_description::FormatItem, OffsetDateTime};

#[derive(Serialize, Deserialize, Clone)]
pub struct RawArticleData {
    pub name: String,
    #[serde(with = "time::serde::iso8601")]
    pub date: OffsetDateTime,
    pub url: String,
    pub intro: Option<String>,
    #[serde(default = "default_branch")]
    pub commit: String,
}

fn default_branch() -> String {
    "main".to_string()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArticleData {
    pub name: String,
    pub date: String,
    pub url: String,
    pub intro: Option<String>,
    pub commit: String,
}

const DATE_TIME_FORMAT: &[FormatItem<'_>] =
    time::macros::format_description!("[year]/[month]/[day]");

impl ArticleData {
    pub fn from_raw(raw: RawArticleData) -> Self {
        Self {
            name: raw.name,
            date: raw.date.format(&DATE_TIME_FORMAT).unwrap(),
            url: raw.url,
            intro: raw.intro,
            commit: raw.commit,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
pub struct BlogParams {
    pub filename: String,
    pub commit: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default, Debug)]
pub struct BlogArticleContent {
    pub title: String,
    pub content: String,
}
