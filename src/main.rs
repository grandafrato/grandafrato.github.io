mod components;
mod pages;

use pages::Page;
use sycamore::view;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            Page {
                p { "Hello" }
            }
        }
    })
}
