use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-menu {
    width: 100%;
    box-sizing: border-box;
    box-shadow: 0 4px 6px var(--menu-bar-box-shadown);
    display: flex;
    align-items: center;
}

.jinya-menu__title {
    font-weight: var(--font-style-bold);
    padding-left: 1rem;
    width: 10vw;
    box-sizing: border-box;
}

.jinya-menu__items-container {
    display: flex;
    width: 80vw;
    margin: 0;
    padding: 0;
}
"
}

pub struct MenuBar {
    children: Children,
    title: &'static str,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuBarProps {
    pub children: Children,
    pub title: &'static str,
}

impl Component for MenuBar {
    type Message = ();
    type Properties = MenuBarProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MenuBar {
            title: props.title,
            children: props.children,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.children = _props.children;
        self.title = _props.title;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-menu">
                <span class="jinya-menu__title">{self.title}</span>
                <ul class="jinya-menu__items-container">
                    {for self.children.iter().enumerate().map(|(_, mut item)| {
                        item
                    })}
                </ul>
            </div>
        }
    }
}