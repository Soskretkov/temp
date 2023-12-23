use bff;
use leptos::*;
mod components;
mod pages;
mod types;
use components::{Footer, Header};
use leptos_router::*;
use pages::{Authorization, Registration, Test, Users};
pub use types::{Session, RoleName};

fn main() {
    leptos::mount_to_body(App);
}

// Условности:
// 1) пока Footer не будет перемонтирован, он не обновит погоду и дату
// рендеринг App не вызовет рендеринга/перемонтирования Footer
// В React, когда родительский перерендеривается, это ведет к перерендерингу всей дочерней иерархии.
// 2) Нет валидации авторизации, нет disabled на кнопку
// 3) Добавить перенаправление на главную если залогиненый попал на страницы аутентификации
// 4) Нет асинхронных запросов у авторизации, регистрации
#[component]
pub fn App() -> impl IntoView {

    let rw_session = create_rw_signal::<Option<Session>>(None);


    view! {
        <Router>
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header rw_user={rw_session}/>
                <main class="mt-[120px]">
                    <Routes>
                        <Route path="/" view=|| view!{<div>"Главная страница"</div>}/>
                        <Route path="/login" view=move || view!{<Authorization rw_session={rw_session}/>}/>
                        <Route path="/register" view=move || view!{<Registration rw_session={rw_session}/>}/>
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
