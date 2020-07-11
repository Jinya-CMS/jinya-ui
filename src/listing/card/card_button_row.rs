use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-card__button-row {
    position: absolute;
    bottom: 0;
    width: 100%;
    opacity: 0;
    transition: opacity 0.3s;
    display: grid;
    grid-auto-flow: column;
}
"
}

pub struct CardButtonRow {
    children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

impl Component for CardButtonRow {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CardButtonRow {
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
            <div class="jinya-card__button-row">
                {for self.children.iter().enumerate().map(|(_, mut button)| {
                    button
                })}
            </div>
        }
    }
}