use crate::{
    types::{ArticleData, BlogArticleContent, BlogParams, RawArticleData},
    RENDERED_LIST, RENDERED_PAGES,
};
use askama::Template;
use axum::{extract::Query, response::Html};
use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ComrakExtensionOptions,
    ComrakOptions, ComrakParseOptions, ComrakPlugins, ComrakRenderOptions, ComrakRenderPlugins,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    route: PageRoute,
}

pub enum PageRoute {
    List { list: List },
    Article { article: Article },
}

impl Index {
    pub fn get_html(&self) -> Html<String> {
        self.render().unwrap_or_default().into()
    }

    pub async fn list() -> Self {
        Self {
            route: PageRoute::List {
                list: List::generate().await,
            },
        }
    }

    pub async fn article(query: Query<BlogParams>) -> Self {
        Self {
            route: PageRoute::Article {
                article: Article::from_query(query).await,
            },
        }
    }
}

#[derive(Template, Clone)]
#[template(path = "list.html")]
pub struct List {
    articles: Vec<ArticleData>,
}

impl List {
    pub fn get_html(&self) -> Html<String> {
        self.render().unwrap_or_default().into()
    }

    pub async fn generate() -> Self {
        async fn update_list() -> (Instant, List) {
            let resp = reqwest::get(
                "https://raw.githubusercontent.com/ming900518/articles/main/article.json",
            )
            .await
            .unwrap();
            let mut fetched_data = resp.json::<Vec<RawArticleData>>().await.unwrap();
            fetched_data.sort_by_key(|x| x.date);
            fetched_data.reverse();
            (
                Instant::now(),
                List {
                    articles: fetched_data
                        .into_iter()
                        .map(ArticleData::from_raw)
                        .collect::<Vec<ArticleData>>(),
                },
            )
        }

        if let Some(mutex) = RENDERED_LIST.get() {
            let mut mutex = mutex.lock().await;
            let (new_instant, new_list) = update_list().await;
            if mutex.0.elapsed() > Duration::from_secs(86400) {
                mutex.0 = new_instant;
                mutex.1 = new_list.clone();
            }
            new_list
        } else {
            let (new_instant, new_list) = update_list().await;
            RENDERED_LIST.get_or_init(|| Mutex::new((new_instant, new_list.clone())));
            new_list
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

    pub async fn from_query(Query(BlogParams { filename, commit }): Query<BlogParams>) -> Self {
        let rendered_pages = RENDERED_PAGES
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .await;
        if let Some(content) = rendered_pages.get(&(filename.clone(), commit.clone())) {
            let content = content.clone();
            drop(rendered_pages);
            Article::success(content.title, content.content)
        } else {
            drop(rendered_pages);
            match reqwest::get(format!(
                "https://raw.githubusercontent.com/ming900518/articles/{commit}/{filename}"
            ))
            .await
            {
                Ok(resp) => {
                    let resp_text = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "載入失敗\n請回上一頁".to_string());
                    let collected_data = resp_text.lines().collect::<Vec<&str>>();
                    let split_data = collected_data
                        .split_first()
                        .unwrap_or((&"載入失敗", &["請回上一頁"]));
                    let title = split_data.0[2..].to_string();

                    let adapter = SyntectAdapter::new(Some("base16-ocean.dark"));

                    let mut plugins = ComrakPlugins::default();
                    plugins.render = {
                        let mut render_plugins = ComrakRenderPlugins::default();
                        render_plugins.codefence_syntax_highlighter = Some(&adapter);
                        render_plugins.heading_adapter = None;
                        render_plugins
                    };

                    let content = markdown_to_html_with_plugins(
                        collected_data.join("\n").trim(),
                        &ComrakOptions {
                            extension: {
                                let mut options = ComrakExtensionOptions::default();
                                options.strikethrough = true;
                                options.table = true;
                                options.tasklist = true;
                                options.superscript = true;
                                options
                            },
                            parse: {
                                let mut options = ComrakParseOptions::default();
                                options.smart = true;
                                options
                            },
                            render: {
                                let mut options = ComrakRenderOptions::default();
                                options.github_pre_lang = true;
                                options.unsafe_ = true;
                                options
                            },
                        },
                        &plugins,
                    );
                    let new_content = BlogArticleContent { title, content };

                    RENDERED_PAGES
                        .get_or_init(|| Mutex::new(HashMap::new()))
                        .lock()
                        .await
                        .insert((filename.clone(), commit.clone()), new_content.clone());

                    Self::success(new_content.title, new_content.content)
                }
                Err(_err) => Self::error(),
            }
        }
    }
}
