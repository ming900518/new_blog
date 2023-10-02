use comrak::plugins::syntect::SyntectAdapter;
use comrak::{
    markdown_to_html_with_plugins, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakPlugins, ComrakRenderOptions, ComrakRenderPlugins,
};
use http::StatusCode;
use leptos::{
    logging::log,
    *,
};
use leptos_meta::Title;
use leptos_router::*;

use crate::types::{BlogArticleContent, BlogParams};

#[component]
pub fn Blog() -> impl IntoView {
    let param = use_params::<BlogParams>();
    let article_content = create_resource(
        move || param.get().ok().and_then(|inner| inner.filename).unwrap(),
        fetch_article_content,
    );

    view! {
        <Transition fallback=move || view!(<span className="loading loading-spinner loading-lg"></span>)>
            {move ||
                if let Some(Ok(article)) = article_content.get() {
                    view!{
                        <>
                            <Title text={format!("{} - Ming Chang", article.title)}/>
                            <div class="card bg-base-100 shadow-xl md:m-5 md:mb-20 lg:ml-20 lg:mr-20 object-fill rounded-none md:rounded-lg">
                                <div class="card-body">
                                    <div class="article-content">
                                        <article id="article-content" inner_html={article.content}/>
                                    </div>
                                </div>
                            </div>
                        </>
                    }.into_view()
                } else {
                    view!{}.into_view()
                }
            }
        </Transition>
    }
}

#[server]
pub async fn fetch_article_content(
    article_filename: String,
) -> Result<BlogArticleContent, ServerFnError> {
    log!("Fetching: {}", article_filename);
    match reqwest::get(format!("https://raw.githubusercontent.com/ming900518/articles/main/{article_filename}")).await {
        Ok(resp) => {
            let resp_text = resp.text().await.unwrap_or_else(|_| "載入失敗\n請回上一頁".to_string());
            let collected_data = resp_text.lines().collect::<Vec<&str>>();
            let split_data = collected_data.split_first().unwrap_or((&"載入失敗", &["請回上一頁"]));
            let title = split_data.0[2..].to_string();

            let content = markdown_to_html_with_plugins(
                    collected_data.join("\n").trim(),
                    &ComrakOptions {
                        extension: ComrakExtensionOptions {
                            strikethrough: true,
                            table: true,
                            tasklist: true,
                            superscript: true,
                            ..ComrakExtensionOptions::default()
                        },
                        parse: ComrakParseOptions {
                            smart: true,
                            ..ComrakParseOptions::default()
                        },
                        render: ComrakRenderOptions {
                            github_pre_lang: true,
                            unsafe_: true,
                            ..ComrakRenderOptions::default()
                        },
                    },
                    &ComrakPlugins {
                        render: ComrakRenderPlugins {
                            codefence_syntax_highlighter: Some(&SyntectAdapter::new(
                                "base16-ocean.dark",
                            )),
                            heading_adapter: None,
                        },
                    },
                );
            Ok(BlogArticleContent {
                title,
                content
            })
        },
        Err(err) => {
            Ok(BlogArticleContent {
                title: format!("錯誤代碼：{}", err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)),
                content: String::from("<p>請確認網址是否正確，網路環境是否暢通<br>如有疑問請<a href=\"mailto:mail@mingchang.tw\">與我聯繫</a></p><p>{}</p>"),
            })
        }
    }
}
