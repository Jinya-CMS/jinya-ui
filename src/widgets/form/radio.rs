use js_sys::*;
use yew::{Callback, Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-radio__color-container--default {
    --state-color: var(--primary-color);
}

.jinya-radio__color-container--negative {
    --state-color: var(--negative-color);
}

.jinya-radio__color-container--positive {
    --state-color: var(--positive-color);
}

.jinya-radio__color-container--disabled {
    --state-color: var(--disabled-border-color);
}

.jinya-radio__input {
    visibility: hidden;
    position: absolute;
}

.jinya-radio__input:invalid {
    outline: none;
    box-shadow: none;
    border: none;
}

.jinya-radio__label {
    display: block;
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    padding-left: 0.25rem;
    padding-right: 0.25rem;
    box-sizing: border-box;
    position: relative;
}

.jinya-radio__label::before,
.jinya-radio__label::after {
    content: '';
    display: none;
}

.jinya-radio__label::before {
    width: 1rem;
    height: 1rem;
    display: inline-block;
    border: 2px solid var(--state-color);
    border-radius: 50%;
    box-sizing: border-box;
    margin-right: 0.5rem;
    background: var(--white);
}

.jinya-radio__label:hover::before {
    background: var(--input-background-color);
}

.jinya-radio__label::after {
    position: absolute;
    top: 5px;
    left: 7px;
    height: 10px;
    width: 10px;
    border-radius: 50%;
    transform: rotate(45deg);
    background: var(--state-color);
}

.jinya-radio__input:checked + .jinya-radio__label::after {
    display: block;
}

.jinya-radio__validation-message {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
}
"
}

#[derive(Clone, PartialEq)]
pub enum RadioState {
    Default,
    Negative,
    Positive,
}

pub struct Radio {
    link: ComponentLink<Self>,
    label: String,
    on_change: Callback<()>,
    state: RadioState,
    validation_message: String,
    checked: bool,
    group: String,
    value: String,
    disabled: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps {
    pub label: String,
    pub on_change: Callback<()>,
    #[prop_or(RadioState::Default)]
    pub state: RadioState,
    #[prop_or("".to_string())]
    pub validation_message: String,
    pub checked: bool,
    pub group: String,
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
}

pub enum Msg {
    Change,
}

impl Default for RadioState {
    fn default() -> Self {
        RadioState::Default
    }
}

impl Radio {
    fn get_radio_container_class(&self) -> String {
        let class = match self.state {
            RadioState::Default => "jinya-radio__color-container--default",
            RadioState::Negative => "jinya-radio__color-container--negative",
            RadioState::Positive => "jinya-radio__color-container--positive",
        }.to_string();

        if self.disabled {
            "jinya-radio__color-container--disabled".to_string()
        } else {
            class
        }
    }
}

impl Component for Radio {
    type Message = Msg;
    type Properties = RadioProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Radio {
            link,
            label: props.label,
            state: props.state,
            validation_message: props.validation_message,
            on_change: props.on_change,
            checked: props.checked,
            group: props.group,
            value: props.value,
            disabled: props.disabled,
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
        self.group = _props.group;
        self.value = _props.value;
        self.disabled = _props.disabled;

        true
    }

    fn view(&self) -> Html {
        let id = super::super::super::id_generator::generate_id();
        html! {
            <div class=self.get_radio_container_class()>
                <div class="jinya-radio__container">
                    <input
                        id=id
                        type="radio"
                        onchange=self.link.callback(|_| Msg::Change)
                        checked=self.checked
                        name=&self.group
                        class="jinya-radio__input"
                        value=&self.value
                        disabled=self.disabled
                    />
                    <label for=id class="jinya-radio__label">{&self.label}</label>
                </div>
                <span class="jinya-radio__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}