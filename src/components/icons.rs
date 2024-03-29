use sycamore::reactive::Scope;
use sycamore::view::View;
use sycamore::web::Html;
use sycamore::{component, view, Prop};

#[derive(Prop)]
pub struct HamburgerIconProps<'a> {
    class: &'a str,
}

#[component]
pub fn HamburgerIcon<'a, G: Html>(cx: Scope<'a>, props: HamburgerIconProps<'a>) -> View<G> {
    view! { cx,
        svg(xmlns="<http://www.w3.org/2000/svg>", class=props.class,
            fill="none", viewBox="0 0 24 24", stroke="currentColor") {
            path(stroke-linecap="round", stroke-linejoin="round",
                 stroke-width="2", d="M4 6h16M4 12h16M4 18h16")
        }
    }
}

#[component]
pub fn ComputerIcon<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        svg(xmlns="http://www.w3.org/2000/svg", viewBox="0 0 24 24",
            fill="currentColor", class="w-6 h-auto") {
            path(d="M16.5 7.5h-9v9h9v-9z")
            path(fill-rule="evenodd", clip-rule="evenodd", d="M8.25 2.25A.75.75 0
                 019 3v.75h2.25V3a.75.75 0 011.5 0v.75H15V3a.75.75 0 011.5
                 0v.75h.75a3 3 0 013 3v.75H21A.75.75 0
                 0121 9h-.75v2.25H21a.75.75 0 010 1.5h-.75V15H21a.75.75 0 010
                 1.5h-.75v.75a3 3 0 01-3 3h-.75V21a.75.75 0 01-1.5
                 0v-.75h-2.25V21a.75.75 0 01-1.5 0v-.75H9V21a.75.75 0 01-1.5
                 0v-.75h-.75a3 3 0 01-3-3v-.75H3A.75.75 0 013
                 15h.75v-2.25H3a.75.75 0 010-1.5h.75V9H3a.75.75 0
                 010-1.5h.75v-.75a3 3 0 013-3h.75V3a.75.75 0 01.75-.75zM6
                 6.75A.75.75 0 016.75 6h10.5a.75.75 0 01.75.75v10.5a.75.75 0
                 01-.75.75H6.75a.75.75 0 01-.75-.75V6.75z")
        }
    }
}
