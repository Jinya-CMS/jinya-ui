use yew::prelude::*;
use yew::{Component, ComponentLink, Html};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-menu {
    width: 100%;
    box-sizing: border-box;
    box-shadow: 0 4px 6px var(--menu-bar-box-shadown);
    display: flex;
}

.jinya-menu__title {
    font-weight: var(--font-style-bold);
    padding-left: 1rem;
    width: 10vw;
    box-sizing: border-box;
    margin-top: auto;
    margin-bottom: auto;
}

.jinya-menu__items-container {
    display: flex;
    width: 80vw;
    margin: 0;
    padding: 0;
}

.jinya-menu__search-bar {
    width: 12%;
    background: var(--input-background-color);
    margin-right: auto;
    margin-left: 0;
    display: flex;
    justify-content: space-between;
    padding-left: 1rem;
    box-sizing: border-box;
}

.jinya-menu__search-input {
    height: 100%;
    width: 100%;
    background: none;
    border: none;
    font-size: 1rem;
    color: var(--primary-color);
    font-family: var(--font-family);
    outline: none;
}

.jinya-menu__search-button {
    font-size: 1.5rem;
    margin-top: auto;
    margin-bottom: auto;
    margin-right: 1rem;
    color: var(--primary-color);
    background: none;
    border: none;
    cursor: pointer;
    outline: none;
}
"
}

pub enum Msg {
    Search(FocusEvent),
    Input(InputData),
}

pub struct MenuBar {
    link: ComponentLink<Self>,
    children: Children,
    title: String,
    search_placeholder: Option<String>,
    search_text: String,
    on_search: Callback<String>,
    on_keyword: Callback<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuBarProps {
    pub children: Children,
    pub title: String,
    #[prop_or(None)]
    pub search_placeholder: Option<String>,
    #[prop_or_default]
    pub on_search: Callback<String>,
    #[prop_or_default]
    pub on_keyword: Callback<String>,
}

impl Component for MenuBar {
    type Message = Msg;
    type Properties = MenuBarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MenuBar {
            link,
            title: props.title,
            children: props.children,
            search_placeholder: props.search_placeholder,
            search_text: "".to_string(),
            on_search: props.on_search,
            on_keyword: props.on_keyword,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(data) => {
                self.search_text = data.value;
                self.on_keyword.emit(self.search_text.to_string());
            }
            Msg::Search(event) => {
                event.prevent_default();
                self.on_search.emit(self.search_text.to_string());
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.children = _props.children;
        self.title = _props.title;
        self.search_placeholder = _props.search_placeholder;
        self.on_search = _props.on_search;
        self.on_keyword = _props.on_keyword;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-menu">
                <span class="jinya-menu__title">{&self.title}</span>
                <ul class="jinya-menu__items-container">
                    {for self.children.iter().enumerate().map(|(_, mut item)| {
                        item
                    })}
                </ul>
                {if self.search_placeholder.is_some() {
                    html! {
                        <form onsubmit=self.link.callback(|event| Msg::Search(event)) class="jinya-menu__search-bar">
                            <input value=self.search_text placeholder=self.search_placeholder.as_ref().unwrap() oninput=self.link.callback(|e| Msg::Input(e)) class="jinya-menu__search-input" type="search" />
                            <button type="submit" class="jinya-menu__search-button">
                                <span class="mdi mdi-magnify"></span>
                            </button>
                        </form>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }
}
