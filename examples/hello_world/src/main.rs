use yew::prelude::*;
use yew_lmth::lmth;

#[function_component]
fn App() -> Html {
    lmth!(
        ! {
            h1 { "Hello, world!" }
            p { "This is an yew app with `lmth!{}` syntax." }
        }
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
