pub mod index;

use sycamore::view;
use sycamore::{component, component::Children, reactive::Scope, view::View, web::Html, Prop};

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
        div(class="px-6") {
            div(class="bg-white mx-auto max-w-screen-lg rounded-lg my-16 p-16") {
                (children)
            }
        }
    }
}
