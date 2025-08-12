use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Horse Sea Men"}</h1>
            <h2>{"Editions"}</h2>
            <p>
            <ul class="editions">
                <Link<Route> to={Route::Editions { edition: "WHY2025".to_string() }}>{ "WHY2025" }</Link<Route>>
            </ul>
            </p>
            <h2>{"The name"}</h2>
            <p>{"Like all good things, there is a story behind the name. You should visit at the next "} <a href="https://2k25.balccon.org/" target="_blank" rel="noopener noreferrer">{"BalCCon"}</a> {" to learn more!"}</p>
        </div>
    }
}
