use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="min-h-screen bg-gray-100 flex items-center justify-center">
            <h1 class="text-3xl font-bold text-blue-600">{ "Hello Yew" }</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
