mod editions;
mod home;

use editions::Editions;
use home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/editions")]
    Editions,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => Home(),
        Route::Editions => Editions(),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
