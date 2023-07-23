use leptos::{
    server_fn::serde::{Deserialize, Serialize},
    *,
};
use time::{format_description::FormatItem, OffsetDateTime};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let article_list = create_resource(cx, || (), |_| async { fetch_article_list().await });

    view! { cx,
        <Suspense fallback=move || view! { cx, <></> }>
            <>
                {move || {
                    article_list.with(cx, |articles| articles
                        .clone()
                        .map(|articles| {
                            articles
                                .into_iter()
                                .map(|article| view! { cx, 
                                    <div class="card bg-base-100 shadow-xl mb-5 w-full">
                                        <div class="card-body">
                                            <div class="flex flex-row">
                                                <h1 class="card-title justify-start grow">{article.name}</h1>
                                                <h2 class="text-sm justify-end">{article.date}</h2>
                                            </div>
                                            <p>{article.intro}</p>
                                        </div>
                                    </div>
                                })
                                .collect_view(cx)
                        })
                    )
                }}
            </>
        </Suspense>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawArticleData {
    name: String,
    #[serde(with = "time::serde::iso8601")]
    date: OffsetDateTime,
    url: String,
    intro: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArticleData {
    name: String,
    date: String,
    url: String,
    intro: String
}

const DATE_TIME_FORMAT: &[FormatItem<'_>] = time::macros::format_description!(
    "[year]/[month]/[day] [hour padding:none repr:12]:[minute padding:none] [period case:upper]"
);

impl ArticleData {
    fn from_raw(raw: RawArticleData) -> Self {
        Self { name: raw.name, date: raw.date.format(&DATE_TIME_FORMAT).unwrap(), url: raw.url, intro: raw.intro }
    }
}


#[server(FetchArticleList, "/api")]
pub async fn fetch_article_list() -> Result<Vec<ArticleData>, ServerFnError> {
    let resp =
        reqwest::get("https://raw.githubusercontent.com/ming900518/articles/main/article.json")
            .await
            .unwrap();
    let mut fetched_data = resp.json::<Vec<RawArticleData>>().await.unwrap();
    fetched_data.sort_by_key(|x| x.date);
    fetched_data.reverse();
    let processed_data = fetched_data.into_iter().map(ArticleData::from_raw).collect::<Vec<ArticleData>>();
    Ok(processed_data)
}
