use sycamore::component::Children;
use sycamore::reactive::{create_signal, ReadSignal, Scope, Signal};
use sycamore::view::View;
use sycamore::web::Html;
use sycamore::{component, view, Prop};

use crate::components::icons::{ComputerIcon, HamburgerIcon};

#[component]
pub fn NavBar<G: Html>(cx: Scope) -> View<G> {
    let visibility = create_signal(cx, false);
    view! { cx,
        nav(class="flex flex-wrap items-center justify-between w-full py-4
                   md:py-0 text-lg text-gray-700 bg-white") {
            div {
                a(href="#", class="flex font-bold text-xl text-center py-auto mx-2") {
                    "Grandafrato"
                    ComputerIcon()
                }
            }
            ToggleButton(visible=visibility)
            NavItems(visible=visibility) {
                NavItem { "Hello" }
                NavItem { "World" }
            }
        }
    }
}

#[derive(Prop)]
struct NavItemsProps<'a, G: Html> {
    children: Children<'a, G>,
    visible: &'a ReadSignal<bool>,
}

#[component]
fn NavItems<'a, G: Html>(cx: Scope<'a>, props: NavItemsProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    view! { cx,
        div(class=format!("{} w-full md:flex md:items-center md:w-auto",
                          if *props.visible.get() { "" } else { "hidden" })) {
            ul(class="text-base text-gray-700 pt-4 md:flex md:justify-between
                      md:pt-0") {
                (children)
            }
        }
    }
}

#[derive(Prop)]
struct NavItemProps<'a, G: Html> {
    children: Children<'a, G>,
    #[builder(default)]
    href: &'a str,
}

#[component]
fn NavItem<'a, G: Html>(cx: Scope<'a>, props: NavItemProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! { cx,
        li {
            a(class="md:p-4 py-2 block hover:text-purple-400", href=props.href) {
                (children)
            }
        }
    }
}

#[derive(Prop)]
struct ToggleButtonProps<'a> {
    visible: &'a Signal<bool>,
}

#[component]
fn ToggleButton<'a, G: Html>(cx: Scope<'a>, props: ToggleButtonProps<'a>) -> View<G> {
    let toggle_visibililty = |_| props.visible.set(!*props.visible.get());

    view! { cx,
        button(on:click=toggle_visibililty, class="md:hidden") {
            HamburgerIcon(class="h-6 w-6 cursor-pointer block mx-2")
        }
    }
}
