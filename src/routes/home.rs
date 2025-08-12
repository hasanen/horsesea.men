use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

pub fn Home() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to Horse Sea Men" }</h1>
            <Link<Route> to={Route::Editions}>{ "click here to go home" }</Link<Route>>
        </div>
    }
}
