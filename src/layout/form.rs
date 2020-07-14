use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-form {
    width: 100%;
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
    display: flex;
    flex-wrap: wrap;
}

.jinya-form > div {
    flex: 0 0 100%;
}
"
}

pub struct Form {
    children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormProps {
    pub children: Children,
}

impl Component for Form {
    type Message = ();
    type Properties = FormProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Form {
            children: props.children,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.children = _props.children;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-form">
                {for self.children.iter().enumerate().map(|(_, mut child)| {
                    child
                })}
            </div>
        }
    }
}