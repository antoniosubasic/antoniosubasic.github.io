use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TaskbarProps<'a, 'b> {
    pub socials: Vec<(&'a str, &'b str)>,
}

pub struct Taskbar {
    props: TaskbarProps<'static, 'static>,
}

pub enum Msg {}

impl Component for Taskbar {
    type Message = Msg;
    type Properties = TaskbarProps<'static, 'static>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.props != *ctx.props() {
            self.props = ctx.props().clone();
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div
                class={classes!(
                    [ "taskbar" ],
                    [ "bg-white", "dark:bg-black" ],
                    [ "text-black", "dark:text-white" ],
                    [ "fixed", "bottom-5", "left-0", "z-10" ],
                    [ "w-full", "flex", "justify-center" ],
                    [ "p-3", "md:p-4" ]
                )}
            >
                <div
                    class={classes!(
                        [ "border", "rounded-full" ],
                        [ "border-neutral-400", "dark:border-neutral-600" ],
                        [ "flex", "items-center", "justify-center" ],
                        [ "gap-6", "md:gap-10" ],
                        [ "p-4", "px-6" ]
                    )}
                >
                    {
                        props.socials.iter().map(|url| {
                            html! {
                                <a
                                    href={url.0}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="hover:opacity-70 transition-opacity"
                                >
                                    <img
                                        src={format!("/images/{}.svg", url.1)}
                                        alt={format!("{} icon", url.1)}
                                        class="w-7 h-7 xl:w-8 xl:h-8 dark:invert"
                                    />
                                </a>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        }
    }
}
