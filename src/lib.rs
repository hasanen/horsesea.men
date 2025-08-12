use crate::routes::{Route, switch};
use yew::prelude::*;
use yew_router::prelude::*;

mod routes;

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
