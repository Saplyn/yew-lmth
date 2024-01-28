use yew::prelude::*;
use yew_lmth::lmth;

#[function_component]
fn App() -> Html {
    let cond = use_state(|| false);

    lmth! {
        ! {
            if *cond {
                p { "True" }
            } else {
                p { "False" }
            }
            button(onclick: move |_| cond.set(!*cond)) { "Toggle" }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
