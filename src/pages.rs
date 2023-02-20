use sycamore::view;
use sycamore::{component, prelude::Children, reactive::Scope, view::View, web::Html, Prop};

use crate::components::nav_bar::NavBar;

#[derive(Prop)]
pub struct PageProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn Page<'a, G: Html>(cx: Scope<'a>, props: PageProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! { cx,
        main {
            NavBar()
            PageContents { (children) }
        }
    }
}

#[derive(Prop)]
struct PageContentsProp<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
fn PageContents<'a, G: Html>(cx: Scope<'a>, props: PageContentsProp<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! { cx,
        div(class="flex bg-white mx-auto max-w-3xl rounded-lg my-16 mb-auto p-16") {
            (children)
        }
    }
}