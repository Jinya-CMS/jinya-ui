use yew::prelude::*;
use yew::{Callback, Component, ComponentLink, Html};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-textarea__color-container {
    width: 100%;
}

.jinya-textarea__color-container--default {
    --state-color: var(--primary-color);
}

.jinya-textarea__color-container--negative {
    --state-color: var(--negative-color);
}

.jinya-textarea__color-container--positive {
    --state-color: var(--positive-color);
}

.jinya-textarea__color-container--disabled {
    --state-color: var(--disabled-border-color);
}

.jinya-textarea__container {
    display: inline-block;
    border: 2px solid var(--state-color);
    border-radius: 5px;
    padding: 0.5rem 0.75rem 0.25rem;
    position: relative;
    margin-top: 0.75rem;
    width: 100%;
    box-sizing: border-box;
}

.jinya-textarea__textarea {
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    border: none;
    padding: 0;
    width: 100%;
    outline: none;
    font-family: var(--font-family);
    resize: vertical;
}

.jinya-textarea__textarea:disabled {
    cursor: not-allowed;
}

.jinya-textarea__textarea:invalid {
    outline: none;
    box-shadow: none;
    border: none;
}

.jinya-textarea__label {
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

.jinya-textarea__validation-message {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
}
"
}

#[derive(Clone, PartialEq)]
pub enum TextareaState {
    Default,
    Negative,
    Positive,
}

pub struct Textarea {
    link: ComponentLink<Self>,
    label: String,
    on_input: Callback<String>,
    state: TextareaState,
    value: String,
    textarea_type: String,
    validation_message: String,
    placeholder: String,
    disabled: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextareaProps {
    pub label: String,
    pub on_input: Callback<String>,
    #[prop_or(TextareaState::Default)]
    pub state: TextareaState,
    pub value: String,
    #[prop_or("text".to_string())]
    pub textarea_type: String,
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

impl Default for TextareaState {
    fn default() -> Self {
        TextareaState::Default
    }
}

impl Textarea {
    fn get_textarea_container_class(&self) -> String {
        let class = match self.state {
            TextareaState::Default => {
                "jinya-textarea__color-container jinya-textarea__color-container--default"
            }
            TextareaState::Negative => {
                "jinya-textarea__color-container jinya-textarea__color-container--negative"
            }
            TextareaState::Positive => {
                "jinya-textarea__color-container jinya-textarea__color-container--positive"
            }
        }
        .to_string();

        if self.disabled {
            "jinya-textarea__color-container jinya-textarea__color-container--disabled".to_string()
        } else {
            class
        }
    }
}

impl Component for Textarea {
    type Message = Msg;
    type Properties = TextareaProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Textarea {
            link,
            label: props.label,
            on_input: props.on_input,
            state: props.state,
            value: props.value,
            textarea_type: props.textarea_type,
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
        self.textarea_type = _props.textarea_type;
        self.placeholder = _props.placeholder;
        self.disabled = _props.disabled;

        true
    }

    fn view(&self) -> Html {
        let id = super::super::super::id_generator::generate_id();
        html! {
            <div class=self.get_textarea_container_class()>
                <div class="jinya-textarea__container">
                    <label for=id class="jinya-textarea__label">{&self.label}</label>
                    <textarea
                        id=id
                        type=self.textarea_type
                        disabled=self.disabled
                        placeholder=self.placeholder
                        oninput=self.link.callback(|e: InputData| Msg::Input(e.value))
                        value=&self.value
                        class="jinya-textarea__textarea"
                        rows=10
                    />
                </div>
                <span class="jinya-textarea__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}
