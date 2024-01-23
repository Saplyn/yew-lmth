use yew::prelude::*;
use yew_lmth::lmth;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn Secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    lmth! {
        div {
            h1 { "Secure" }
            button ( onclick ) { "Go Home" }
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => lmth! {
            ! {
                h1 { "Home" }
                a ( href="/secure", style="display: block;" ) { "Go to secure page" }
                a ( href="/404", style="display: block;"  ) { "Go to 404 page" }
            }
        },
        Route::Secure => lmth! { Secure },
        Route::NotFound => lmth! { h1 { "404" } },
    }
}

#[function_component]
fn App() -> Html {
    lmth! {
        BrowserRouter {
            Switch<Route> ( render: switch )
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
