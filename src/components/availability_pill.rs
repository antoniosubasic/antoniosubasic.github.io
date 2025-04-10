use gloo_timers::callback::Interval;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AvailabilityPillProps {
    pub text_available: String,
    pub text_unavailable: String,
    pub available: bool,
    pub year: u16,
}

pub enum Msg {
    TogglePulse,
}

pub struct AvailabilityPill {
    is_pulsing: bool,
    animation_duration: u16,
    _interval: Option<Interval>,
}

impl Component for AvailabilityPill {
    type Message = Msg;
    type Properties = AvailabilityPillProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut component = Self {
            is_pulsing: false,
            animation_duration: 1000,
            _interval: None,
        };

        if ctx.props().available {
            let link = ctx.link().clone();
            let interval = Interval::new(component.animation_duration.into(), move || {
                link.send_message(Msg::TogglePulse);
            });

            component._interval = Some(interval);
        }

        component
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TogglePulse => {
                self.is_pulsing = !self.is_pulsing;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div
                class={classes!(
                    "inline-flex",
                    "items-center",
                    "px-4",
                    "py-2",
                    "rounded-full",
                    "font-medium",
                    "select-none",
                    "relative",
                    if props.available {
                        ["bg-green-50", "text-green-700", "dark:bg-green-900", "dark:text-green-300"]
                    } else {
                        ["bg-yellow-100", "text-yellow-600", "dark:bg-yellow-900", "dark:text-yellow-200"]
                    }
                )}
            >
                <div
                    class={classes!(
                        "inline-block",
                        "w-2",
                        "h-2",
                        "mr-2",
                        "rounded-full",
                        "relative",
                        if props.available {
                            ["bg-green-700", "dark:bg-green-300"]
                        } else {
                            ["bg-yellow-600", "dark:bg-yellow-200"]
                        }
                    )}
                >
                    if props.available && self.is_pulsing {
                        <div
                            class={classes!(
                                "absolute",
                                "w-2",
                                "h-2",
                                "rounded-full",
                                "bg-black",
                                "bg-opacity-40",
                                "dark:bg-white",
                                "dark:bg-opacity-40",
                                "animate-ping"
                            )}
                            style={format!(
                                "animation-duration: {}ms; animation-iteration-count: 1;",
                                self.animation_duration
                            )}
                        ></div>
                    }
                </div>
                <span class="text-[1.05rem]">
                    { if props.available { &props.text_available } else { &props.text_unavailable } }
                    { format!(" {}", props.year) }
                </span>
            </div>
        }
    }
}
