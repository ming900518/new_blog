use leptos::*;
use leptos_meta::Title;
use leptos_router::A;

use crate::types::{ArticleData, RawArticleData};

#[component]
pub fn Home() -> impl IntoView {
    let article_list_request = create_resource(|| (), move |_| fetch_article_list());

    view! {
        <Transition fallback=move || ()>
            {move || {
                view! {
                    <Title text="Ming Chang"/>
                    <div class="p-5">
                    {
                        match article_list_request.get().transpose() {
                            Ok(Some(articles)) => {
                                articles
                                    .into_iter()
                                    .map(|article| view! {
                                        <div class="card bg-base-100 shadow-xl mb-5 lg:ml-20 lg:mr-20 rounded-lg select-none cursor-pointer hover:bg-base-300">
                                            <A href={format!("/blog/?filename={}&commit={}", article.url, article.commit)}>
                                                <div class="card-body">
                                                    <div class="flex lg:flex-row flex-col gap-2">
                                                        <h1 class="card-title justify-start grow">{&article.name}</h1>
                                                        <h2 class="text-sm justify-end">{&article.date}</h2>
                                                    </div>
                                                    <p class={
                                                        let article_intro = article.clone().intro;
                                                        move || if article_intro.is_none() {"hidden"} else {""}
                                                    }>{article.intro.unwrap_or_default()}</p>
                                                </div>
                                            </A>
                                        </div>
                                    })
                                    .collect_view()
                            },
                            Err(error) => {
                                vec![
                                    view!{
                                        <div class="card bg-base-100 shadow-xl mb-5 w-full rounded-lg select-none cursor-pointer hover:bg-base-300">
                                            <div class="card-body">
                                                <div class="flex lg:flex-row flex-col gap-2">
                                                    <h1 class="card-title justify-start grow">"發生錯誤"</h1>
                                                    <h2 class="text-sm justify-end"></h2>
                                                </div>
                                                <p>{format!("{error:?}")}</p>
                                            </div>
                                        </div>
                                    }
                                ].collect_view()
                            }
                            _ => {
                                vec![view!{}].collect_view()
                            }
                        }
                    }
                    </div>
                }
            }}
        </Transition>
    }
}

#[server]
pub async fn fetch_article_list() -> Result<Vec<ArticleData>, ServerFnError> {
    let resp =
        reqwest::get("https://raw.githubusercontent.com/ming900518/articles/main/article.json")
            .await
            .unwrap();
    let mut fetched_data = resp.json::<Vec<RawArticleData>>().await.unwrap();
    fetched_data.sort_by_key(|x| x.date);
    fetched_data.reverse();
    let processed_data = fetched_data
        .into_iter()
        .map(ArticleData::from_raw)
        .collect::<Vec<ArticleData>>();
    Ok(processed_data)
}
