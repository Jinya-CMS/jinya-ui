use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-floating-action-button {
    box-sizing: border-box;
    border-radius: 50%;
    transition: background 0.3s, color 0.3s;
    border: 2px solid var(--primary-color);
    font-size: 2rem;
    width: 4rem;
    height: 4rem;
    cursor: pointer;
    background: var(--primary-color);
    color: var(--white);
    position: fixed;
    right: 3rem;
    bottom: 3rem;
}

.jinya-floating-action-button:hover {
    background: var(--white);
    color: var(--primary-color);
}
"
}

pub type Fab = FloatingActionButton;

pub struct FloatingActionButton {
    link: ComponentLink<Self>,
    icon: String,
    on_click: Callback<()>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FloatingActionButtonProps {
    pub icon: String,
    pub on_click: Callback<()>,
}

pub enum Msg {
    Click
}

impl FloatingActionButton {
    fn get_icon_class(&self) -> String {
        format!("mdi mdi-{}", self.icon)
    }
}

impl Component for FloatingActionButton {
    type Message = Msg;
    type Properties = FloatingActionButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FloatingActionButton {
            link,
            icon: props.icon,
            on_click: props.on_click,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                self.on_click.emit(());
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.icon = _props.icon;
        self.on_click = _props.on_click;

        true
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::Click) class="jinya-floating-action-button">
                <span class=self.get_icon_class()></span>
            </button>
        }
    }
}