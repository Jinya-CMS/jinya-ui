use yew::{Component, ComponentLink, Html};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-page {
    margin-left: 10%;
    margin-right: 10%;
}
"
}

pub struct Page {
    link: ComponentLink<Self>,
    children: ChildrenRenderer<VNode>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageProps {
    pub children: ChildrenRenderer<VNode>
}

impl Component for Page {
    type Message = ();
    type Properties = PageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Page {
            link,
            children: props.children,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.children = _props.children;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-page">
                {for self.children.iter().enumerate().map(|(_, mut child)| {
                    child
                })}
            </div>
        }
    }
}