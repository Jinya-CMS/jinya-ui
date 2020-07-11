use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-card__header {
    font-size: var(--font-size-24);
    line-height: var(--line-height-34);
    padding: 0.5rem;
    border: 2px solid var(--primary-color);
    border-top-left-radius: 5px;
    border-top-right-radius: 5px;
    border-bottom: none;
    transition: all 0.3s;
    box-sizing: border-box;
}
"
}

pub struct CardHeader {
    title: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

impl Component for CardHeader {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CardHeader {
            title: props.title,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.title = _props.title;

        true
    }

    fn view(&self) -> Html {
        html! {
            <header class="jinya-card__header">
                {&self.title}
            </header>
        }
    }
}
