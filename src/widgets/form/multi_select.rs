use wasm_bindgen::__rt::core::iter::{Filter, Map};
use wasm_bindgen::__rt::core::slice::Iter;
use yew::{Callback, Component, ComponentLink, Html};
use yew::prelude::*;
use std::borrow::Borrow;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-multi-select__color-container--default {
    --state-color: var(--primary-color);
}

.jinya-multi-select__color-container--negative {
    --state-color: var(--negative-color);
}

.jinya-multi-select__color-container--positive {
    --state-color: var(--positive-color);
}

.jinya-multi-select__color-container--disabled {
    --state-color: var(--disabled-border-color);
}

.jinya-multi-select__container {
    display: inline-block;
    border: 2px solid var(--state-color);
    border-radius: 5px;
    padding: 0.5rem 0.75rem 0.25rem;
    position: relative;
    margin-top: 0.75rem;
    width: 100%;
}

.jinya-multi-select__item-holder {
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    border: none;
    padding: 0;
    width: 100%;
    display: flex;
    gap: 0.5rem;
    align-items: center;
}

.jinya-multi-select__item-holder:disabled {
    cursor: not-allowed;
}

.jinya-multi-select__item-holder:invalid {
    outline: none;
    box-shadow: none;
    border: none;
}

.jinya-multi-select__label {
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

.jinya-multi-select__validation-message {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
}

.jinya-multi-select__chip {
    display: inline-block;
    font-size: var(--font-size-12);
    color: var(--state-color);
    border: 2px solid var(--state-color);
    position: relative;
    border-radius: 5px;
    line-height: var(--line-height-18);
    padding: 0 0.25rem;
}

.jinya-multi-select__search-field {
    flex: 1;
    padding: 0;
    margin: 0;
    border: 0;
    font-size: var(--font-size-16);
    color: var(--state-color);
    font-family: var(--font-family);
}
"
}

#[derive(Clone, PartialEq)]
pub enum MultiSelectState {
    Default,
    Negative,
    Positive,
}

pub struct MultiSelect {
    link: ComponentLink<Self>,
    label: String,
    on_select: Callback<String>,
    on_deselect: Callback<String>,
    state: MultiSelectState,
    validation_message: String,
    placeholder: String,
    disabled: bool,
    options: Vec<MultiSelectItem>,
    selected_items: Vec<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MultiSelectProps {
    pub label: String,
    #[prop_or_default]
    pub on_select: Callback<String>,
    #[prop_or_default]
    pub on_deselect: Callback<String>,
    #[prop_or(MultiSelectState::Default)]
    pub state: MultiSelectState,
    #[prop_or("".to_string())]
    pub validation_message: String,
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(false)]
    pub disabled: bool,
    pub options: Vec<MultiSelectItem>,
    pub selected_items: Vec<String>,
}

pub enum Msg {
    Select(String),
    Remove(String),
}

impl Default for MultiSelectState {
    fn default() -> Self {
        MultiSelectState::Default
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MultiSelectItem {
    pub value: String,
    pub text: String,
}

impl MultiSelect {
    fn get_multi_select_container_class(&self) -> String {
        let class = match self.state {
            MultiSelectState::Default => "jinya-multi-select__color-container--default",
            MultiSelectState::Negative => "jinya-multi-select__color-container--negative",
            MultiSelectState::Positive => "jinya-multi-select__color-container--positive",
        }.to_string();

        if self.disabled {
            "jinya-multi-select__color-container--disabled".to_string()
        } else {
            class
        }
    }
}

struct Chip {
    link: ComponentLink<Self>,
    value: String,
    text: String,
    on_remove: Callback<String>,
}

enum ChipMsg {
    Remove,
}

#[derive(Clone, PartialEq, Properties)]
struct ChipProps {
    pub value: String,
    pub text: String,
    pub on_remove: Callback<String>,
}

impl Component for Chip {
    type Message = ChipMsg;
    type Properties = ChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Chip {
            link,
            text: props.text,
            value: props.value,
            on_remove: props.on_remove,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            ChipMsg::Remove => self.on_remove.emit(self.value.to_string())
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-multi-select__chip">
                {&self.text}
                <a href="#" class="mdi mdi-close" onclick=self.link.callback(|_| ChipMsg::Remove)></a>
            </div>
        }
    }
}

impl Component for MultiSelect {
    type Message = Msg;
    type Properties = MultiSelectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MultiSelect {
            link,
            label: props.label,
            on_select: props.on_select,
            on_deselect: props.on_deselect,
            state: props.state,
            validation_message: props.validation_message,
            placeholder: props.placeholder,
            disabled: props.disabled,
            options: props.options,
            selected_items: props.selected_items,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(value) => {
                self.on_select.emit(value);
            }
            Msg::Remove(value) => {
                self.on_deselect.emit(value);
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.label = _props.label;
        self.state = _props.state;
        self.validation_message = _props.validation_message;
        self.placeholder = _props.placeholder;
        self.disabled = _props.disabled;
        self.options = _props.options;
        self.on_deselect = _props.on_deselect;
        self.on_select = _props.on_select;
        self.selected_items = _props.selected_items;

        true
    }

    fn view(&self) -> Html {
        let id = super::super::super::id_generator::generate_id();
        html! {
            <div class=self.get_multi_select_container_class()>
                <div class="jinya-multi-select__container">
                    <label for=id class="jinya-multi-select__label">{&self.label}</label>
                    <div
                        id=id
                        disabled=self.disabled
                        placeholder=self.placeholder
                        class="jinya-multi-select__item-holder"
                    >
                        {for self.options.iter().enumerate().map(|(_, mut item)| {
                            if (self.selected_items.contains(&item.value)) {
                                html! {
                                    <Chip text=&item.text value=&item.value on_remove=self.link.callback(|value| Msg::Remove(value)) />
                                }
                            } else {
                                html! {}
                            }
                        })}
                        <input class="jinya-multi-select__search-field" />
                    </div>
                </div>
                <span class="jinya-multi-select__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}
