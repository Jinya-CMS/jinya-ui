use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-card__container {
    width: 100%;
    display: flex;
    flex-flow: row wrap;
    gap: 1rem;
}
"
}

pub struct CardContainer {
    children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

pub enum Msg {
    Click,
}

impl Component for CardContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CardContainer {
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
            <div class="jinya-card__container">
                {for self.children.iter().enumerate().map(|(_, mut card)| {
                    card
                })}
            </div>
        }
    }
}