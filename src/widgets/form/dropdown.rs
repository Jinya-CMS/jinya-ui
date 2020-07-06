use yew::{Callback, Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-dropdown__color-container--default {
    --state-color: var(--primary-color);
    --select-arrow: var(--background-image-select-primary);
}

.jinya-dropdown__color-container--negative {
    --state-color: var(--negative-color);
    --select-arrow: var(--background-image-select-negative);
}

.jinya-dropdown__color-container--positive {
    --state-color: var(--positive-color);
    --select-arrow: var(--background-image-select-positive);
}

.jinya-dropdown__color-container--disabled {
    --state-color: var(--disabled-border-color);
    --select-arrow: var(--background-image-select-disabled);
}

.jinya-dropdown__container {
    display: inline-block;
    border: 2px solid var(--state-color);
    border-radius: 5px;
    padding: 0.5rem 0.75rem 0.25rem;
    position: relative;
    margin-top: 0.75rem;
    width: 100%;
}

.jinya-dropdown__select:disabled {
    cursor: not-allowed;
}

.jinya-dropdown__select {
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    border: none;
    padding: 0;
    width: 100%;
    /* for Firefox */
    -moz-appearance: none;
    /* for Chrome */
    -webkit-appearance: none;
    background-image: var(--select-arrow);
    background-repeat: no-repeat;
    background-position-x: right;
    background-position-y: center;
}

.jinya-dropdown__select:invalid {
    outline: none;
    box-shadow: none;
    border: none;
}

.jinya-dropdown__label {
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

.jinya-dropdown__validation-message {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
}
"
}

#[derive(Clone, PartialEq)]
pub enum DropdownState {
    Default,
    Negative,
    Positive,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItem {
    pub value: String,
    pub text: String,
}

pub struct Dropdown {
    link: ComponentLink<Self>,
    label: String,
    on_select: Callback<String>,
    state: DropdownState,
    dropdown_type: String,
    validation_message: String,
    items: Vec<DropdownItem>,
    placeholder: Option<String>,
    disabled: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownProps {
    pub label: String,
    pub on_select: Callback<String>,
    #[prop_or(DropdownState::Default)]
    pub state: DropdownState,
    #[prop_or("text".to_string())]
    pub dropdown_type: String,
    #[prop_or("".to_string())]
    pub validation_message: String,
    pub items: Vec<DropdownItem>,
    #[prop_or(None)]
    pub placeholder: Option<String>,
    #[prop_or(false)]
    pub disabled: bool,
}

pub enum Msg {
    Change(ChangeData),
}

impl Default for DropdownState {
    fn default() -> Self {
        DropdownState::Default
    }
}

impl Dropdown {
    fn get_dropdown_container_class(&self) -> String {
        let class = match self.state {
            DropdownState::Default => "jinya-dropdown__color-container--default",
            DropdownState::Negative => "jinya-dropdown__color-container--negative",
            DropdownState::Positive => "jinya-dropdown__color-container--positive",
        }.to_string();

        if self.disabled {
            "jinya-dropdown__color-container--disabled".to_string()
        } else {
            class
        }
    }
}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = DropdownProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Dropdown {
            link,
            label: props.label,
            on_select: props.on_select,
            state: props.state,
            placeholder: props.placeholder,
            dropdown_type: props.dropdown_type,
            validation_message: props.validation_message,
            items: props.items,
            disabled: props.disabled,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(data) => {
                let value = match data {
                    ChangeData::Select(element) => element.value(),
                    _ => unreachable!()
                };
                self.on_select.emit(value);
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.label = _props.label;
        self.on_select = _props.on_select;
        self.state = _props.state;
        self.placeholder = _props.placeholder;
        self.validation_message = _props.validation_message;
        self.dropdown_type = _props.dropdown_type;
        self.items = _props.items;
        self.disabled = _props.disabled;

        true
    }

    fn view(&self) -> Html {
        let id = super::super::super::id_generator::generate_id();
        html! {
            <div class=self.get_dropdown_container_class()>
                <div class="jinya-dropdown__container">
                    <label for=id class="jinya-dropdown__label">{&self.label}</label>
                    <select class="jinya-dropdown__select" disabled=self.disabled onchange=self.link.callback(|data: ChangeData| Msg::Change(data))>
                    {if self.placeholder.is_some() {
                        html! {
                            <option value="">{self.placeholder.as_ref().unwrap()}</option>
                        }
                    } else {
                        html! {}
                    }}
                    {for self.items.iter().map(|mut item| {
                        html! {
                            <option value=&item.value>{&item.text}</option>
                        }
                    })}
                    </select>
                </div>
                <span class="jinya-dropdown__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}