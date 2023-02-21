use crate::pages::index::Index;
use crate::pages::Page;
use sycamore::view;
use sycamore::{component, reactive::ReadSignal, reactive::Scope, view::View, web::Html};
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[not_found]
    NotFound,
}

#[component]
pub fn RootComponent<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    Page {
                        (match route.get().as_ref() {
                            AppRoutes::Index => view! { cx, Index() },
                            AppRoutes::NotFound => view! { cx, "404 Not Found" }
                        })
                    }
                }
            }
        )
    }
}
