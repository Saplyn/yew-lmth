use yew::prelude::*;
use yew_lmth::lmth;

#[function_component]
fn App() -> Html {
    let items = (1..=10).collect::<Vec<_>>();

    lmth! {
        ! {
            h2 { "List rendering. `for` syntax is not supported" }
            ul {
                { items.iter().map(|item| lmth! {
                    li { {item} }
                }).collect::<Html>() }
            }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
