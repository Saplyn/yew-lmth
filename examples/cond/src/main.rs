use yew::prelude::*;
use yew_lmth::lmth;

#[function_component]
fn App() -> Html {
    let cond = use_state(|| false);
    let some_text = use_state(|| Some("there's some text".to_string()));

    lmth! {
        ! {
            if *cond {
                p { "True" }
            } else {
                p { "False" }
            }

            if let Some(text) = &*some_text {
                p { {text} }
            } else {
                p { "None" }
            }

            button(onclick: move |_| {
                cond.set(!*cond);
                if some_text.is_some() {
                    some_text.set(None);
                } else {
                    some_text.set(Some("there's some text".to_string()));
                }
            }) { "Toggle" }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
