use yew::{Component, ComponentLink, Html};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VComp, VNode};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-button-row {
    width: 100%;
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
    display: flex;
}

.jinya-button-row--align-end {
    justify-content: flex-end;
}

.jinya-button-row--align-end .jinya-button {
    margin-left: 1rem;
}

.jinya-button-row--align-start {
    justify-content: flex-start;
}

.jinya-button-row--align-start .jinya-button {
    margin-right: 1rem;
}
    "
}

#[derive(Clone, PartialEq)]
pub enum ButtonRowAlignment {
    End,
    Start,
}

pub struct ButtonRow {
    link: ComponentLink<Self>,
    children: ChildrenRenderer<VNode>,
    alignment: ButtonRowAlignment,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonRowProps {
    pub children: ChildrenRenderer<VNode>,
    #[prop_or(ButtonRowAlignment::End)]
    pub alignment: ButtonRowAlignment,
}

impl ButtonRow {
    fn get_alignment_class(&self) -> &str {
        match self.alignment {
            ButtonRowAlignment::End => "jinya-button-row jinya-button-row--align-end",
            ButtonRowAlignment::Start => "jinya-button-row jinya-button-row--align-start"
        }
    }
}

impl Component for ButtonRow {
    type Message = ();
    type Properties = ButtonRowProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonRow {
            link,
            children: props.children,
            alignment: props.alignment,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.children = _props.children;
        self.alignment = _props.alignment;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.get_alignment_class()>
                {for self.children.iter().enumerate().map(|(_, mut button)| {
                    button
                })}
            </div>
        }
    }
}