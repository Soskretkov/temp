use bff;
use leptos::*;
mod components;
mod pages;
mod types;
use bff::server::Session;
use components::{Footer, Header};
use leptos_router::*;
use pages::{Authorization, Registration, Test, Users};
use types::{GlobContext, UserInfo};

fn main() {
    leptos::mount_to_body(App);
}

// Условности:
// 1) Нет валидации авторизации, нет disabled на кнопку
// 2) Нет асинхронных запросов у авторизации, регистрации, users
#[component]
pub fn App() -> impl IntoView {
    let (session, set_session) = create_signal::<Option<Session>>(None);
    let (location, set_location) = create_signal("/".to_string());

    provide_context(GlobContext {
        location,
        session,
        user_info: UserInfo::new(session, location), // Header, Users
    });

    view! {
        <Router>
            { update_location_on_navigation(set_location) }
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_session={set_session}/> // btn. "выход" сбросывает rw_session на None
                <main class="mt-[120px]">
                    <Routes>
                        <Route path="/" view=|| view!{<div>"Главная страница"</div>}/>
                        <Route path="/login" view=move || view!{<Authorization set_session={set_session}/>}/>
                        <Route path="/register" view=move || view!{<Registration set_session={set_session}/>}/>
                        <Route path="/users" view=|| view!{<Users/>}/>
                        <Route path="/post" view=move || view!{<Test/>}/>
                        <Route path="/post/:postId" view=|| view!{<div>"Статья"</div>}/>
                        <Route path="/*" view=|| view!{<div>"Ошибка"</div>}/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

fn update_location_on_navigation(set_location: WriteSignal<String>) -> impl Fn() {
    let current_path = use_location().pathname;
    let (is_app_start, set_is_app_start) = create_signal(true);

    move || {
        if !is_app_start.get() {
            logging::log!("Переход на страницу: {}", current_path.get());
            set_location.set(current_path.get());
        } else {
            current_path.track();
            set_is_app_start.set(false);
        }
    }
}
