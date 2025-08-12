use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

pub fn editions(edition: &str) -> Html {
    let pics: HashMap<&str, HashMap<&str, Vec<&str>>> = HashMap::from([(
        "WHY2025",
        HashMap::from([(
            "pics",
            vec!["/pics/why2025/why2025.png", "/pics/why2025/why2025_2.jpeg"],
        )]),
    )]);

    if let Some(edition_cfg) = pics.get(&edition) {
        let empty_vec = Vec::new();
        let edition_pics = edition_cfg.get("pics").unwrap_or(&empty_vec);

        html! {
            <div>
                <h1>{ edition }</h1>
                <Link<Route> to={Route::Home}>{ "Back" }</Link<Route>>
                <div class="edition-pics">
                { for edition_pics.iter().map(|pic| html! { <p><img src={*pic} /></p> }) }
                </div>
            </div>
        }
    } else {
        html! { <h1>{ "Edition not found" }</h1> }
    }
}
