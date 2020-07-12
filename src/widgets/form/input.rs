use yew::{Callback, Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-input__color-container--default {
    --state-color: var(--primary-color);
}

.jinya-input__color-container--negative {
    --state-color: var(--negative-color);
}

.jinya-input__color-container--positive {
    --state-color: var(--positive-color);
}

.jinya-input__color-container--disabled {
    --state-color: var(--disabled-border-color);
}

.jinya-input__container {
    display: inline-block;
    border: 2px solid var(--state-color);
    border-radius: 5px;
    padding: 0.5rem 0.75rem 0.25rem;
    position: relative;
    margin-top: 0.75rem;
    width: 100%;
    box-sizing: border-box;
}

.jinya-input__input {
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    border: none;
    padding: 0;
    width: 100%;
}

.jinya-input__input:disabled {
    cursor: not-allowed;
}

.jinya-input__input:invalid {
    outline: none;
    box-shadow: none;
    border: none;
}

.jinya-input__label {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
    position: absolute;
    top: -0.75rem;
    background: var(--white);
    padding-left: 0.25rem;
    padding-right: 0.25rem;
    box-sizing: border-box;
    left: 0.5rem;
    z-index: 0;
}

.jinya-input__validation-message {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
}
"
}

#[derive(Clone, PartialEq)]
pub enum InputState {
    Default,
    Negative,
    Positive,
}

pub struct Input {
    link: ComponentLink<Self>,
    label: String,
    on_input: Callback<String>,
    state: InputState,
    value: String,
    input_type: String,
    validation_message: String,
    placeholder: String,
    disabled: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputProps {
    pub label: String,
    pub on_input: Callback<String>,
    #[prop_or(InputState::Default)]
    pub state: InputState,
    pub value: String,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or("".to_string())]
    pub validation_message: String,
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(false)]
    pub disabled: bool,
}

pub enum Msg {
    Input(String),
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Default
    }
}

impl Input {
    fn get_input_container_class(&self) -> String {
        let class = match self.state {
            InputState::Default => "jinya-input__color-container--default",
            InputState::Negative => "jinya-input__color-container--negative",
            InputState::Positive => "jinya-input__color-container--positive",
        }.to_string();

        if self.disabled {
            "jinya-input__color-container--disabled".to_string()
        } else {
            class
        }
    }
}

impl Component for Input {
    type Message = Msg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Input {
            link,
            label: props.label,
            on_input: props.on_input,
            state: props.state,
            value: props.value,
            input_type: props.input_type,
            validation_message: props.validation_message,
            placeholder: props.placeholder,
            disabled: props.disabled,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(value) => {
                self.on_input.emit(value);
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.label = _props.label;
        self.on_input = _props.on_input;
        self.state = _props.state;
        self.value = _props.value;
        self.validation_message = _props.validation_message;
        self.input_type = _props.input_type;
        self.placeholder = _props.placeholder;
        self.disabled = _props.disabled;

        true
    }

    fn view(&self) -> Html {
        let id = super::super::super::id_generator::generate_id();
        html! {
            <div class=self.get_input_container_class()>
                <div class="jinya-input__container">
                    <label for=id class="jinya-input__label">{&self.label}</label>
                    <input
                        id=id
                        type=self.input_type
                        disabled=self.disabled
                        placeholder=self.placeholder
                        oninput=self.link.callback(|e: InputData| Msg::Input(e.value))
                        value=self.value class="jinya-input__input"
                    />
                </div>
                <span class="jinya-input__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}