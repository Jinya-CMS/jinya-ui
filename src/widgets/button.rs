use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-button {
    padding: 0.25rem 0.5rem;
    background: var(--color-two);
    border: 2px solid var(--color-one);
    box-sizing: border-box;
    border-radius: 5px;
    transition: background 0.3s;
    color: var(--color-one);
    font-size: 1rem;
    font-family: var(--font-family);
    cursor: pointer;
}

.jinya-button:hover {
    background: var(--color-one);
    color: var(--color-two);
}

.jinya-button--small {
    font-size: 12px;
}

.jinya-button--primary {
    --color-one: var(--primary-color);
    --color-two: var(--white);
}

.jinya-button--secondary {
    --color-one: var(--secondary-color);
    --color-two: var(--white);
}

.jinya-button--negative {
    --color-one: var(--negative-color);
    --color-two: var(--white);
}

.jinya-button--positive {
    --color-one: var(--positive-color);
    --color-two: var(--white);
}

.jinya-button--information {
    --color-one: var(--information-color);
    --color-two: var(--white);
}

"
}

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Negative,
    Positive,
    Information,
}

pub struct Button {
    link: ComponentLink<Self>,
    label: String,
    button_type: ButtonType,
    on_click: Callback<()>,
    small: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub button_type: ButtonType,
    pub label: String,
    pub on_click: Callback<()>,
    #[prop_or(false)]
    pub small: bool,
}

pub enum Msg {
    Click
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Primary
    }
}

impl Button {
    fn get_button_type_class(&self) -> String {
        let button_class = match self.button_type {
            ButtonType::Primary => "jinya-button jinya-button--primary",
            ButtonType::Secondary => "jinya-button jinya-button--secondary",
            ButtonType::Negative => "jinya-button jinya-button--negative",
            ButtonType::Positive => "jinya-button jinya-button--positive",
            ButtonType::Information => "jinya-button jinya-button--information",
        };

        if self.small {
            format!("{} jinya-button--small", button_class)
        } else {
            button_class.to_string()
        }
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            label: props.label,
            on_click: props.on_click,
            button_type: props.button_type,
            small: props.small,
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
        self.label = _props.label;
        self.on_click = _props.on_click;
        self.button_type = _props.button_type;
        self.small = _props.small;

        true
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::Click) class=self.get_button_type_class()>
                {&self.label}
            </button>
        }
    }
}