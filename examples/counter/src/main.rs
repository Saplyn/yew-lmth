use yew::prelude::*;
use yew_lmth::lmth;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    lmth!(
        div {
            button ( onclick ) { "+1" }
            p { {*counter} }
        }
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
