use yew::{Callback, Children};
use yew::prelude::*;

use crate::widgets::button::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-dialog__backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--dropback);
    opacity: 0;
    display: none;
    transition: opacity 0.3s;
}

.jinya-dialog__backdrop--open {
    opacity: 0.8;
    display: block;
}

.jinya-dialog {
    border-radius: 5px;
    box-shadow: 0 0 10px var(--menu-bar-box-shadown);
    opacity: 0;
    display: none;
    transition: opacity 0.3s;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: var(--white);
    width: 40rem;
}

.jinya-dialog--open {
    opacity: 1;
    display: block;
}

.jinya-dialog__content {
    padding: 1rem;
}

.jinya-dialog__header {
    font-size: var(--font-size-32);
    line-height: var(--line-height-46);
    padding: 1rem;
    border-bottom: 2px solid var(--primary-color);
}

.jinya-dialog__footer {
    display: flex;
    justify-content: space-between;
    padding: 1rem;
    border-top: 2px solid var(--primary-color);
    gap: 1rem;
}

.jinya-dialog__content p {
    margin: 0;
}
"
}

#[derive(Clone, PartialEq)]
pub enum DialogType {
    Primary,
    Negative,
}

pub struct Dialog {
    link: ComponentLink<Self>,
    title: String,
    children: Children,
    on_primary: Callback<()>,
    on_secondary: Callback<()>,
    primary_label: String,
    secondary_label: String,
    is_open: bool,
    dialog_type: DialogType,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub children: Children,
    pub on_primary: Callback<()>,
    pub on_secondary: Callback<()>,
    pub primary_label: String,
    pub secondary_label: String,
    pub is_open: bool,
    pub dialog_type: DialogType,
}

pub enum Msg {
    Primary,
    Secondary,
}

impl Component for Dialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Dialog {
            link,
            title: props.title,
            children: props.children,
            on_primary: props.on_primary,
            on_secondary: props.on_secondary,
            primary_label: props.primary_label,
            secondary_label: props.secondary_label,
            is_open: props.is_open,
            dialog_type: props.dialog_type,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Primary => self.on_primary.emit(()),
            Msg::Secondary => self.on_secondary.emit(())
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.on_secondary = _props.on_secondary;
        self.on_primary = _props.on_primary;
        self.children = _props.children;
        self.title = _props.title;
        self.is_open = _props.is_open;
        self.primary_label = _props.primary_label;
        self.secondary_label = _props.secondary_label;
        self.dialog_type = _props.dialog_type;

        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class=self.get_backdrop_class()></div>
                <div class=self.get_dialog_class()>
                    <header class="jinya-dialog__header">{&self.title}</header>
                    <div class="jinya-dialog__content">
                        {for self.children.iter().enumerate().map(|(_, mut item)| {
                            item
                        })}
                    </div>
                    <div class="jinya-dialog__footer">
                        <Button button_type=self.get_secondary_button_type() label=&self.secondary_label on_click=self.link.callback(|_| Msg::Secondary) />
                        <Button button_type=self.get_primary_button_type() label=&self.primary_label on_click=self.link.callback(|_| Msg::Primary) />
                    </div>
                </div>
            </>
        }
    }
}

impl Dialog {
    fn get_secondary_button_type(&self) -> ButtonType {
        match self.dialog_type {
            DialogType::Primary => ButtonType::Secondary,
            DialogType::Negative => ButtonType::Positive,
        }
    }

    fn get_primary_button_type(&self) -> ButtonType {
        match self.dialog_type {
            DialogType::Primary => ButtonType::Primary,
            DialogType::Negative => ButtonType::Negative,
        }
    }

    fn get_backdrop_class(&self) -> String {
        if self.is_open {
            "jinya-dialog__backdrop jinya-dialog__backdrop--open".to_string()
        } else {
            "jinya-dialog__backdrop".to_string()
        }
    }

    fn get_dialog_class(&self) -> String {
        if self.is_open {
            "jinya-dialog jinya-dialog--open".to_string()
        } else {
            "jinya-dialog".to_string()
        }
    }
}