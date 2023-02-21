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
        div(class="px-6 flex justify-center") {
            div(class="bg-white mx-auto md:mr-8 md:ml-auto min-w-fit max-w-screen-xl rounded-lg
                       my-16 p-16") {
                (children)
            }
            SideInfo()
        }
    }
}

#[component]
fn SideInfo<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="bg-white mr-auto h-fit min-w-fit max-w-screen-lg rounded-lg
               mt-20 px-16 py-2 hidden md:block") { p { "Hello" } }
    }
}
