use yew::prelude::*;
use yew::{Component, ComponentLink, Html};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-split-view {
    display: grid;
    width: 100%;
    grid-template-columns: 1fr 2px 1fr;
    grid-gap: 1rem;
    gap: 1rem;
    padding: 0.5rem;
}

.jinya-split-view__left {
    grid-column: 1;
    max-height: calc(100vh - 9rem + var(--line-height-23));
    overflow-y: auto;
}

.jinya-split-view__middle-bar {
    background: var(--primary-color);
    height: calc(100vh - 9rem + var(--line-height-23));
    display: block;
    width: 2px;
    grid-column: 2;
}

.jinya-split-view__right {
    grid-column: 3;
    max-height: calc(100vh - 9rem + var(--line-height-23));
    overflow-y: auto;
    padding: 0.5rem;
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
                <div class="jinya-split-view__middle-bar"/>
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
