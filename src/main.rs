mod components;
mod pages;
mod router;

use router::RootComponent;
use sycamore::view;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    sycamore::render(|cx| {
        view! { cx, RootComponent() }
    })
}
