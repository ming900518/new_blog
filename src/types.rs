use leptos::Params;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use time::{format_description::FormatItem, OffsetDateTime};

#[derive(Serialize, Deserialize, Clone)]
pub struct RawArticleData {
    pub name: String,
    #[serde(with = "time::serde::iso8601")]
    pub date: OffsetDateTime,
    pub url: String,
    pub intro: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArticleData {
    pub name: String,
    pub date: String,
    pub url: String,
    pub intro: Option<String>,
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
        }
    }
}

#[derive(Params, PartialEq, Clone)]
pub struct BlogParams {
    pub filename: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct BlogArticleContent {
    pub title: String,
    pub content: String,
}
