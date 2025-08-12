mod editions;
mod home;

use editions::editions;
use home::home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/editions/:edition")]
    Editions { edition: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::Editions { edition } => editions(&edition),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
