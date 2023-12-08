use leptos::*;
use crate::components::Icon;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="flex -mt-3">
            <Icon id="fa-code" class="text-[70px] mr-[10px]}"/>
            <div class="">
                <div class="text-[48px] font-semibold leading-[48px] mt-4">"Блог"</div>
                <div class="text-[18px] font-bold">"веб-разработка"</div>
            </div>
        </div>
    }
}
