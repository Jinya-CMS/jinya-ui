use yew::prelude::*;
use yew::{Component, ComponentLink, Html};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-split-view {
    display: grid;
    width: 100%;
    grid-template-columns: 50% 50%;
    grid-gap: 1rem;
    gap: 1rem;
}

.jinya-split-view__left {
    grid-column: 1;
}

.jinya-split-view__right {
    grid-column: 2;
}
"
}

pub struct SplitView {
    left: Html,
    right: Html,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageProps {
    pub left: Html,
    pub right: Html,
}

impl Component for SplitView {
    type Message = ();
    type Properties = PageProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SplitView {
            left: props.left,
            right: props.right,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.right = props.right;
        self.left = props.left;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-split-view">
                <div class="jinya-split-view__left">
                    {self.get_left()}
                </div>
                <div class="jinya-split-view__right">
                    {self.get_right()}
                </div>
            </div>
        }
    }
}

impl SplitView {
    fn get_left(&self) -> Children {
        Children::new(vec![self.left.clone()])
    }
    fn get_right(&self) -> Children {
        Children::new(vec![self.right.clone()])
    }
}
