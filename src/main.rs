use leptos::*;
mod components;
mod pages;
mod server;
mod types;
mod utils;
use components::{Footer, Header};
use leptos_router::*;
use pages::{Authorization, Registration, Users};
use types::{AuthedUser, GlobContext};

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let (authed_user, set_authed_user) = create_signal::<Option<AuthedUser>>(None);

    provide_context(GlobContext {
        // session: Authorization, Registration, struct UserInfo
        // user_data: Header, PageGuard, Users
        authed_user: authed_user,
        roles: create_action(move |_: &()| async move { server::fetch_all_roles().await }),
    });

    view! {
        <Router>
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_authed_user={set_authed_user}/> // btn. "выход" сбрасывает authed_user на None
                <main class="pt-[120px]">
                    <Routes>
                        <Route path="/" view=move || { view!{<div>"Главная страница"</div>} }/>
                        <Route path="/login" view=move || { view!{<Authorization set_authed_user={set_authed_user}/>} }/>
                        <Route path="/register" view=move || { view!{<Registration set_authed_user={set_authed_user}/>} }/>
                        <Route path="/users" view=move || { Users }/>
                        <Route path="/post" view=move || { view!{<div>"Статьи"</div>} }/>
                        <Route path="/post/:postId" view=move || { view!{<div>"Статья"</div>} }/>
                        <Route path="/*" view=move || { view!{<div>"Ошибка"</div>} }/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
