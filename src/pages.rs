use crate::types::{ArticleData, RawArticleData};
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    route: PageRoute,
}

pub enum PageRoute {
    List,
    Article { filename: String, commit: String },
}

impl Index {
    pub fn get_html(&self) -> Html<String> {
        self.render().unwrap_or_default().into()
    }

    pub const fn list() -> Self {
        Self {
            route: PageRoute::List,
        }
    }

    pub const fn article(filename: String, commit: String) -> Self {
        Self {
            route: PageRoute::Article { filename, commit },
        }
    }
}

#[derive(Template)]
#[template(path = "list.html")]
pub struct List {
    articles: Vec<ArticleData>,
}

impl List {
    pub fn get_html(&self) -> Html<String> {
        self.render().unwrap_or_default().into()
    }

    pub async fn prepare_data() -> Self {
        let resp =
            reqwest::get("https://raw.githubusercontent.com/ming900518/articles/main/article.json")
                .await
                .unwrap();
        let mut fetched_data = resp.json::<Vec<RawArticleData>>().await.unwrap();
        fetched_data.sort_by_key(|x| x.date);
        fetched_data.reverse();
        Self {
            articles: fetched_data
                .into_iter()
                .map(ArticleData::from_raw)
                .collect::<Vec<ArticleData>>(),
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
            title: String::from("錯誤代碼"),
            content: String::from("<p>請確認網址是否正確，網路環境是否暢通<br>如有疑問請<a href=\"mailto:mail@mingchang.tw\">與我聯繫</a></p><p>{}</p>")
        }
    }
}
