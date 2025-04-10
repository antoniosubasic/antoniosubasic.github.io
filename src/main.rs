use yew::prelude::*;

mod components;
use components::availability_pill::AvailabilityPill;

#[function_component]
fn App() -> Html {
    html! {
        <div class="min-h-screen flex flex-col gap-4 items-center justify-center">
            <div class="flex flex-col gap-2 items-center">
                <AvailabilityPill
                    available={false}
                    text_available="Open for Internships"
                    text_unavailable="Employed for Internship Position"
                    year={2025}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
