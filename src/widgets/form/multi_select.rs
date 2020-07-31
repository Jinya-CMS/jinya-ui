use std::rc::Rc;

use yew::prelude::*;
use yew::services::ConsoleService;
use yew::{Callback, Component, ComponentLink, Html};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-multi-select__color-container {
    width: 100%;
}

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
    box-sizing: border-box;
}

.jinya-multi-select__item-holder {
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    border: none;
    padding: 0;
    width: 100%;
    display: flex;
    align-items: center;
    flex-wrap: wrap;
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
    margin-right: 0.5rem;
}

.jinya-multi-select__search-field {
    flex: 1;
    padding: 0;
    margin: 0;
    border: 0;
    font-size: var(--font-size-16);
    color: var(--state-color);
    font-family: var(--font-family);
    background: var(--white);
    outline: none;
}

.jinya-multi-select__dropdown {
    position: absolute;
    display: none;
    width: calc(100% - 10px);
    background: var(--white);
    border: 2px solid var(--state-color);
    left: 2px;
    top: 2.25rem;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
    max-height: 15rem;
    overflow-y: auto;
}

.jinya-multi-select__search-field:focus + .jinya-multi-select__dropdown,
.jinya-multi-select__dropdown--open {
    display: block;
}

.jinya-multi-select__dropdown-item {
    padding: 0.25rem 0.5rem;
}

.jinya-multi-select__dropdown-item:hover {
    background: var(--input-background-color);
    cursor: pointer;
}
"
}

struct Chip {
    link: ComponentLink<Self>,
    item: MultiSelectItem,
    on_remove: Callback<MultiSelectItem>,
}

enum ChipMsg {
    Remove,
}

#[derive(Clone, PartialEq, Properties)]
struct ChipProps {
    pub item: MultiSelectItem,
    pub on_remove: Callback<MultiSelectItem>,
}

impl Component for Chip {
    type Message = ChipMsg;
    type Properties = ChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Chip {
            link,
            item: props.item,
            on_remove: props.on_remove,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            ChipMsg::Remove => self.on_remove.emit(self.item.clone()),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-multi-select__chip">
                {&self.item.text}
                <a href="#" class="mdi mdi-close" onclick=self.link.callback(|_| ChipMsg::Remove)></a>
            </div>
        }
    }
}

struct DropdownItem {
    link: ComponentLink<Self>,
    item: MultiSelectItem,
    on_select: Callback<MultiSelectItem>,
}

enum DropdownItemMsg {
    Select,
}

#[derive(Clone, PartialEq, Properties)]
struct DropdownItemProps {
    pub item: MultiSelectItem,
    pub on_select: Callback<MultiSelectItem>,
}

impl Component for DropdownItem {
    type Message = DropdownItemMsg;
    type Properties = DropdownItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DropdownItem {
            link,
            item: props.item,
            on_select: props.on_select,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            DropdownItemMsg::Select => {
                ConsoleService::log("selected");
                self.on_select.emit(self.item.clone());
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-multi-select__dropdown-item" onclick=self.link.callback(|_| DropdownItemMsg::Select)>
                {&self.item.text}
            </div>
        }
    }
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
    on_select: Callback<MultiSelectItem>,
    on_deselect: Callback<MultiSelectItem>,
    on_filter: Callback<String>,
    state: MultiSelectState,
    validation_message: String,
    placeholder: String,
    disabled: bool,
    options: Vec<MultiSelectItem>,
    selected_items: Vec<MultiSelectItem>,
    flyout_open: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MultiSelectProps {
    pub label: String,
    pub on_select: Callback<MultiSelectItem>,
    pub on_deselect: Callback<MultiSelectItem>,
    #[prop_or_default]
    pub on_filter: Callback<String>,
    #[prop_or(MultiSelectState::Default)]
    pub state: MultiSelectState,
    #[prop_or("".to_string())]
    pub validation_message: String,
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(false)]
    pub disabled: bool,
    pub options: Vec<MultiSelectItem>,
    pub selected_items: Vec<MultiSelectItem>,
}

pub enum Msg {
    Select(MultiSelectItem),
    Remove(MultiSelectItem),
    Filter(String),
    CloseFlyout,
    OpenFlyout,
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
            on_filter: props.on_filter,
            flyout_open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(value) => {
                ConsoleService::log(format!("{} {}", value.text, value.value).as_str());
                self.on_select.emit(value);
            }
            Msg::Remove(value) => {
                self.on_deselect.emit(value);
            }
            Msg::Filter(value) => {
                self.on_filter.emit(value);
            }
            Msg::CloseFlyout => {
                self.flyout_open = false;
            }
            Msg::OpenFlyout => {
                self.flyout_open = true;
            }
        }

        true
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
            <div class=self.get_multi_select_container_class() onmouseleave=self.link.callback(|_| Msg::CloseFlyout)>
                <div class="jinya-multi-select__container">
                    <label for=id class="jinya-multi-select__label">{&self.label}</label>
                    <div disabled=self.disabled placeholder=self.placeholder class="jinya-multi-select__item-holder">
                        {for self.selected_items.iter().enumerate().map(|(_, mut item)| {
                            let key = Rc::new(format!("chip-{}", item.value));
                            html! {
                                <Chip key=key item=item on_remove=self.link.callback(|value| Msg::Remove(value)) />
                            }
                        })}
                        <input
                            id=id
                            class="jinya-multi-select__search-field"
                            oninput=self.link.callback(|e: InputData| Msg::Filter(e.value))
                            onfocus=self.link.callback(|_| Msg::OpenFlyout)
                            placeholder=self.get_placeholder()
                        />
                        <div class=self.get_flyout_class()>
                            {for self.options.iter().enumerate().map(|(_, mut item)| {
                                let key = Rc::new(format!("item-{}", item.value));
                                html! {
                                    <DropdownItem key=key item=item on_select=self.link.callback(|value| Msg::Select(value)) />
                                }
                            })}
                        </div>
                    </div>
                </div>
                <span class="jinya-multi-select__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}

impl MultiSelect {
    fn get_placeholder(&self) -> String {
        let placeholder = &self.placeholder;
        if self.selected_items.is_empty() {
            placeholder.to_string()
        } else {
            "".to_string()
        }
    }

    fn get_flyout_class(&self) -> String {
        if self.flyout_open {
            "jinya-multi-select__dropdown jinya-multi-select__dropdown--open".to_string()
        } else {
            "jinya-multi-select__dropdown".to_string()
        }
    }

    fn get_multi_select_container_class(&self) -> String {
        let class = match self.state {
            MultiSelectState::Default => {
                "jinya-multi-select__color-container jinya-multi-select__color-container--default"
            }
            MultiSelectState::Negative => {
                "jinya-multi-select__color-container jinya-multi-select__color-container--negative"
            }
            MultiSelectState::Positive => {
                "jinya-multi-select__color-container jinya-multi-select__color-container--positive"
            }
        }
        .to_string();

        if self.disabled {
            "jinya-multi-select__color-container jinya-multi-select__color-container--disabled"
                .to_string()
        } else {
            class
        }
    }
}
