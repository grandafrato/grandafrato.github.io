mod components;

use components::nav_bar::NavBar;
use sycamore::view;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            NavBar
            p { "A website." }
        }
    })
}
