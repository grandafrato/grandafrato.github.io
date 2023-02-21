use sycamore::view;
use sycamore::{component, reactive::Scope, view::View, web::Html};

#[component]
pub fn Index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        h1(class="text-3xl") { "Hello there!" }
        p(class="text-base") {
            "This is my main page. There isn't much here yet, but soon there"
            " will be."
        }
    }
}
