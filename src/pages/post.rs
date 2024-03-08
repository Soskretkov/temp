mod comments;
use crate::components::{Icon, PageErrMsg, H2};
use crate::server::{self};
use comments::Comments;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Post() -> impl IntoView {
    view! {
        <div class="px-20 my-10">
            <Await
                future=move|| {
                    let post_id = use_params_map().with(|params| params.get("id").cloned()).unwrap();
                    async move { server::fetch_post(&post_id).await }
                }
                let: post_wrapped
            >{
                match post_wrapped {
                    Ok(post) => {
                        let title = post.title.clone();
                        let created_at = post.created_at.format("%Y-%m-%d").to_string();
                        view! {
                            <div>
                                <img class="float-left mr-5 mb-2.5" src={&post.image_url} alt={&post.title}/>
                                <H2>{title}</H2>
                                // верстка: special-panel у автора
                                <div class="flex justify-between mt-[-20px] mb-5">
                                    <div class="flex"> // верстка: published_at у автора
                                        <Icon id="fa-calendar-o" class="relative top-[-1px] text-[18px] mr-[7px]"/>
                                        <div class = "text-[18px]">{created_at}</div>
                                    </div>
                                    <div class="flex"> // верстка: buttons у автора
                                        <Icon id="fa-pencil-square-o" class="text-[21px] mr-[10px]"/>
                                        <Icon id="fa-trash-o" class="text-[21px]"/>
                                    </div>
                                </div>
                                <div class="text-[18px]">{&post.content}</div>
                            </div>
                            <Comments post_id={"".to_string()} comments={post.comments.clone()}/>
                        }
                        .into_view()
                    },
                    Err(e) => {
                        let err_msg = e.to_string();
                        view! {<PageErrMsg>{err_msg.clone()}</PageErrMsg>}
                    },
                }
            }</Await>
        </div>
    }
}
