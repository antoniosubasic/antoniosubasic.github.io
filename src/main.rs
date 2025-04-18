use yew::prelude::*;

mod components;
use components::availability_pill::AvailabilityPill;
use components::taskbar::Taskbar;

#[function_component]
fn App() -> Html {
    html! {
        <main
            class={classes!(
                [ "max-w-[23.15rem] sm:max-w-xl md:max-w-3xl xl:max-w-7xl" ],
                [ "flex flex-col gap-8 items-center justify-center" ]
            )}
        >
            <AvailabilityPill available={false} year={2025} />
            <h1
                class={classes!(
                    [ "text-[2.5rem] sm:text-[2.8rem] md:text-5xl xl:text-6xl" ],
                    [ "text-center font-medium leading-[1.1]" ]
                )}
            >
                { "I'm " }
                <span class="underline-gradient">{ "Antonio" }</span>
                { ", computer science student based in " }
                <span class="text-gradient">{ "Austria" }</span>
            </h1>
            <p
                class={classes!(
                    [ "sm:text-md xl:text-lg" ],
                    [ "max-w-[20rem] sm:max-w-lg md:max-w-2xl xl:max-w-6xl" ],
                    [ "text-center text-stone-600 dark:text-stone-400" ]
                )}
            >
                {
                    "A passionate Computer Science student
                    specializing in software development and project management,
                    with a focus on building efficient, scalable applications
                    and crafting innovative digital solutions."
                }
            </p>
            <Taskbar
                socials={vec![
                    ("https://linkedin.com/in/subasicantonio", "linkedin"),
                    ("https://github.com/antoniosubasic", "github"),
                    ("mailto:antonio.subasic.public@gmail.com", "email"),
                ]}
            />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
