use comrak::plugins::syntect::SyntectAdapter;
use comrak::{
    markdown_to_html_with_plugins, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakPlugins, ComrakRenderOptions, ComrakRenderPlugins,
};
use http::StatusCode;
use leptos::{
    server_fn::serde::{Deserialize, Serialize},
    *,
};
use leptos_meta::Title;
use leptos_router::*;

#[derive(Params, PartialEq, Clone)]
struct BlogParams {
    filename: Option<String>,
}

#[component]
pub fn Blog() -> impl IntoView {
    let param = use_params::<BlogParams>();
    let article_content = create_blocking_resource(
        move || param.get().unwrap().filename.unwrap(),
        fetch_article_content,
    );

    view! {
        <Suspense fallback=move || ()>
            {move ||
                match article_content.get().transpose() {
                    Ok(article) => {
                        let article = article.unwrap_or_default();
                        view!{
                            <>
                            <Title text={format!("{} - Ming Chang", article.title)}/>
                            <div class="card bg-base-100 shadow-xl md:m-5 object-fill rounded-none md:rounded-lg">
                                <div class="card-body">
                                    <div class="article-content">
                                        <article id="article-content" inner_html={article.content}/>
                                    </div>
                                </div>
                            </div>
                            </>
                        }
                    }
                    Err(error) =>
                        view!{
                            <>
                                <div class="card bg-base-100 shadow-xl md:m-5 object-fill rounded-none md:rounded-lg">
                                    <div class="card-body">
                                        <div class="article-content">
                                            <article id="article-content">
                                                <h1>"發生錯誤"</h1>
                                                <p>{format!("{error:?}")}</p>
                                            </article>
                                        </div>
                                    </div>
                                </div>
                            </>
                        }
                    }
                }
        </Suspense>
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BlogArticleContent {
    title: String,
    content: String,
}

#[server(FetchArticleContent, "/api")]
pub async fn fetch_article_content(
    article_filename: String,
) -> Result<BlogArticleContent, ServerFnError> {
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
