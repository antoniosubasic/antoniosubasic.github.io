use yew::prelude::*;

mod components;
use components::availability_pill::AvailabilityPill;

#[function_component]
fn App() -> Html {
    html! {
        <main class="max-h-screen max-w-[23.15rem] sm:max-w-xl md:max-w-3xl xl:max-w-7xl flex flex-col gap-8 items-center justify-center">
            <AvailabilityPill
                available={false}
                text_available="Open for Internships"
                text_unavailable="Employed for Internship Position"
                year={2025}
            />
            <h1 class="text-[2.35rem] sm:text-[2.8rem] md:text-5xl xl:text-6xl text-center font-medium leading-[1.1]">
                { "I'm " }
                <span class="underline-gradient">{ "Antonio" }</span>
                { ", computer science student based in " }
                <span class="text-gradient">{ "Austria" }</span>
            </h1>
            <p class="sm:text-md xl:text-lg max-w-[20rem] sm:max-w-lg md:max-w-2xl xl:max-w-6xl text-center text-stone-400">
                {
                    "A passionate Computer Science student
                    specializing in software development and project management,
                    with a focus on building efficient, scalable applications
                    and crafting innovative digital solutions."
                }
            </p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
