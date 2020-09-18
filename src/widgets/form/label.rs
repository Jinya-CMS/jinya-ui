use yew::prelude::*;
use yew::{Component, ComponentLink, Html};

pub fn get_css() -> &'static str {
    // language=CSS
    "
.jinya-label {
    flex: 0 0 100%;
}
"
}

pub struct Label {
    label: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LabelProps {
    pub label: String,
}

impl Component for Label {
    type Message = ();
    type Properties = LabelProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Label { label: props.label }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.label = props.label;

        true
    }

    fn view(&self) -> Html {
        html! {
            <label class="jinya-label">{&self.label}</label>
        }
    }
}
