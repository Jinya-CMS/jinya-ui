use yew::prelude::*;
use yew::{Callback, Component, ComponentLink, Html};

use crate::widgets::button::ButtonType;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-card__button {
    display: flex;
    height: 2.5rem;
    box-sizing: border-box;
    border: 2px solid var(--color-one);
    background: var(--color-one);
    transition: background 0.3s, color 0.3s;
    color: var(--color-two);
    justify-content: center;
    align-items: center;
    font-size: var(--font-size-24);
    cursor: pointer;
}

.jinya-card__button:first-child {
    border-bottom-left-radius: 5px;
}

.jinya-card__button:last-child {
    border-bottom-right-radius: 5px;
}

.jinya-card__button:hover {
    background: var(--color-two);
    color: var(--color-one);
}

.jinya-card__button--primary {
    --color-one: var(--primary-color);
    --color-two: var(--white);
}

.jinya-card__button--secondary {
    --color-one: var(--secondary-color);
    --color-two: var(--white);
}

.jinya-card__button--negative {
    --color-one: var(--negative-color);
    --color-two: var(--white);
}

.jinya-card__button--positive {
    --color-one: var(--positive-color);
    --color-two: var(--white);
}

.jinya-card__button--information {
    --color-one: var(--information-color);
    --color-two: var(--white);
}
"
}

#[derive(Clone)]
pub struct CardButton {
    link: ComponentLink<Self>,
    icon: String,
    tooltip: String,
    on_click: Callback<()>,
    button_type: ButtonType,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub icon: String,
    pub tooltip: String,
    pub on_click: Callback<()>,
    #[prop_or(ButtonType::Primary)]
    pub button_type: ButtonType,
}

pub enum Msg {
    Click,
}

impl Component for CardButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CardButton {
            link,
            icon: props.icon,
            tooltip: props.tooltip,
            on_click: props.on_click,
            button_type: props.button_type,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => self.on_click.emit(()),
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.icon = _props.icon;
        self.tooltip = _props.tooltip;
        self.on_click = _props.on_click;
        self.button_type = _props.button_type;

        true
    }

    fn view(&self) -> Html {
        html! {
            <button title=self.tooltip onclick=self.link.callback(|_| Msg::Click) class=self.get_button_class()>
                <span class=self.get_icon_class()></span>
            </button>
        }
    }
}

impl CardButton {
    fn get_icon_class(&self) -> String {
        format!("mdi mdi-{}", self.icon)
    }

    fn get_button_class(&self) -> String {
        let button_class = match self.button_type {
            ButtonType::Primary => "jinya-card__button jinya-card__button--primary",
            ButtonType::Secondary => "jinya-card__button jinya-card__button--secondary",
            ButtonType::Negative => "jinya-card__button jinya-card__button--negative",
            ButtonType::Positive => "jinya-card__button jinya-card__button--positive",
            ButtonType::Information => "jinya-card__button jinya-card__button--information",
        };

        button_class.to_string()
    }
}
