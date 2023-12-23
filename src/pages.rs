use crate::types::{ArticleData, BlogParams, RawArticleData, Theme};
use askama::Template;
use axum::{extract::Query, response::Html};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    theme: Theme,
    route: PageRoute,
}

pub enum PageRoute {
    List { articles: Vec<ArticleData> },
    Article { filename: String, commit: String },
}

impl Index {
    pub fn get_html(&self) -> Html<String> {
        self.render().unwrap_or_default().into()
    }

    pub async fn list(theme: Theme) -> Self {
        let resp =
            reqwest::get("https://raw.githubusercontent.com/ming900518/articles/main/article.json")
                .await
                .unwrap();
        let mut fetched_data = resp.json::<Vec<RawArticleData>>().await.unwrap();
        fetched_data.sort_by_key(|x| x.date);
        fetched_data.reverse();

        Self {
            theme,
            route: PageRoute::List {
                articles: fetched_data
                    .into_iter()
                    .map(ArticleData::from_raw)
                    .collect::<Vec<ArticleData>>(),
            },
        }
    }

    pub fn article(theme: Theme, BlogParams { filename, commit }: BlogParams) -> Self {
        Self {
            theme,
            route: PageRoute::Article { filename, commit },
        }
    }
}

#[derive(Template)]
#[template(path = "article.html")]
pub struct Article {
    title: String,
    content: String,
}

impl Article {
    pub fn get_html(&self) -> Html<String> {
        self.render().unwrap_or_default().into()
    }

    pub const fn success(title: String, content: String) -> Self {
        Self { title, content }
    }

    pub fn error() -> Self {
        Self {
            title: String::from("錯誤"),
            content: String::from("<p>請確認網址是否正確，網路環境是否暢通<br>如有疑問請<a href=\"mailto:mail@mingchang.tw\">與我聯繫</a></p><p>{}</p>")
        }
    }
}
