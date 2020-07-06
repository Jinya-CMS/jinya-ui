use js_sys::*;
use yew::{Callback, Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-checkbox__color-container--default {
    --state-color: var(--primary-color);
}

.jinya-checkbox__color-container--negative {
    --state-color: var(--negative-color);
}

.jinya-checkbox__color-container--positive {
    --state-color: var(--positive-color);
}

.jinya-checkbox__container {
}

.jinya-checkbox__input {
    visibility: hidden;
    position: absolute;
}

.jinya-checkbox__input:invalid {
    outline: none;
    box-shadow: none;
    border: none;
}

.jinya-checkbox__label {
    display: block;
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    padding-left: 0.25rem;
    padding-right: 0.25rem;
    box-sizing: border-box;
    position: relative;
}

.jinya-checkbox__label::before,
.jinya-checkbox__label::after {
    content: '';
    display: none;
}

.jinya-checkbox__label::before {
    width: 1rem;
    height: 1rem;
    display: inline-block;
    border: 2px solid var(--state-color);
    border-radius: 5px;
    box-sizing: border-box;
    margin-right: 0.5rem;
    background: var(--white);
}

.jinya-checkbox__label:hover::before {
    background: var(--input-background-color);
}

.jinya-checkbox__label::after {
    position: absolute;
    top: 4px;
    left: 10px;
    border-bottom: 2px solid var(--state-color);
    height: 8px;
    width: 4px;
    border-right: 2px solid var(--state-color);
    transform: rotate(45deg);
}

.jinya-checkbox__input:checked + .jinya-checkbox__label::after {
    display: block;
}

.jinya-checkbox__validation-message {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
}
"
}

#[derive(Clone, PartialEq)]
pub enum CheckboxState {
    Default,
    Negative,
    Positive,
}

pub struct Checkbox {
    link: ComponentLink<Self>,
    label: String,
    on_change: Callback<()>,
    state: CheckboxState,
    validation_message: String,
    checked: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CheckboxProps {
    pub label: String,
    pub on_change: Callback<()>,
    #[prop_or(CheckboxState::Default)]
    pub state: CheckboxState,
    #[prop_or("".to_string())]
    pub validation_message: String,
    pub checked: bool,
}

pub enum Msg {
    Change,
}

impl Default for CheckboxState {
    fn default() -> Self {
        CheckboxState::Default
    }
}

impl Checkbox {
    fn get_checkbox_container_class(&self) -> String {
        match self.state {
            CheckboxState::Default => "jinya-checkbox__color-container--default",
            CheckboxState::Negative => "jinya-checkbox__color-container--negative",
            CheckboxState::Positive => "jinya-checkbox__color-container--positive",
        }.to_string()
    }
}

impl Component for Checkbox {
    type Message = Msg;
    type Properties = CheckboxProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Checkbox {
            link,
            label: props.label,
            state: props.state,
            validation_message: props.validation_message,
            on_change: props.on_change,
            checked: props.checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Change => {
                self.on_change.emit(());
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.label = _props.label;
        self.state = _props.state;
        self.validation_message = _props.validation_message;
        self.on_change = _props.on_change;
        self.checked = _props.checked;

        true
    }

    fn view(&self) -> Html {
        let id = super::super::super::id_generator::generate_id();
        html! {
            <div class=self.get_checkbox_container_class()>
                <div class="jinya-checkbox__container">
                    <input
                        id=id
                        type="checkbox"
                        onchange=self.link.callback(|_| Msg::Change)
                        checked=self.checked
                        class="jinya-checkbox__input"
                    />
                    <label for=id class="jinya-checkbox__label">{&self.label}</label>
                </div>
                <span class="jinya-checkbox__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}