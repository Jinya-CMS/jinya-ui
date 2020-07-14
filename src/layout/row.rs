use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-row {
    width: 100%;
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
    display: flex;
    flex-wrap: wrap;
}

.jinya-row--align-end {
    justify-content: flex-end;
}

.jinya-row--align-start {
    justify-content: flex-start;
}

.jinya-row--align-center {
    justify-content: center;
}

.jinya-row--align-space-between {
    justify-content: space-between;
}

.jinya-row--align-stretch {
    justify-content: stretch;
}
"
}

#[derive(Clone, PartialEq)]
pub enum RowAlignment {
    End,
    Start,
    SpaceBetween,
    Center,
    Stretch,
}

pub struct Row {
    children: Children,
    alignment: RowAlignment,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RowProps {
    pub children: Children,
    #[prop_or(RowAlignment::Start)]
    pub alignment: RowAlignment,
}

impl Row {
    fn get_alignment_class(&self) -> &str {
        match self.alignment {
            RowAlignment::End => "jinya-row jinya-row--align-end",
            RowAlignment::Start => "jinya-row jinya-row--align-start",
            RowAlignment::SpaceBetween => "jinya-row jinya-row--align-space-between",
            RowAlignment::Center => "jinya-row jinya-row--align-center",
            RowAlignment::Stretch => "jinya-row jinya-row--align-stretch",
        }
    }
}

impl Component for Row {
    type Message = ();
    type Properties = RowProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Row {
            children: props.children,
            alignment: props.alignment,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
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
                {for self.children.iter().enumerate().map(|(_, mut child)| {
                    child
                })}
            </div>
        }
    }
}