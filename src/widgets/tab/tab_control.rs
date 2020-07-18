use uuid::Uuid;
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-tab {
    width: 100%;
    background: var(--white);
}

.jinya-tab__link-list {
    display: flex;
    gap: 1rem;
    background: var(--white);
    width: 100%;
    border-bottom: 2px solid var(--primary-color);
    padding-bottom: 0.25rem;
}

.jinya-tab__link {
    font-size: var(--font-size-24);
    color: var(--primary-color);
    background: var(--white);
    font-weight: var(--font-style-light);
    opacity: 0.5;
    transition: opacity 0.3s;
    cursor: pointer;
}

.jinya-tab__link--active {
    opacity: 1;
}

.jinya-tab__pages {
    background: var(--white);
}

.jinya-tab__page {
    display: none;
}

.jinya-tab__page--active {
    display: block;
}
"
}

#[derive(PartialEq, Clone)]
pub struct TabPage {
    pub title: String,
    pub content: Html,
    uniq_id: Uuid,
}

impl TabPage {
    pub fn new(title: String, content: Html) -> Self {
        TabPage {
            title,
            content,
            uniq_id: Uuid::new_v4(),
        }
    }
}

pub struct TabControl {
    link: ComponentLink<Self>,
    pages: Vec<TabPage>,
    active_page_uuid: Uuid,
}

#[derive(PartialEq, Clone, Properties)]
pub struct TabControlProps {
    pub pages: Vec<TabPage>,
}

pub enum Msg {
    ChangeTab(usize),
}

impl Component for TabControl {
    type Message = Msg;
    type Properties = TabControlProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let active_page = props.pages.first().unwrap().uniq_id;

        TabControl {
            link,
            pages: props.pages,
            active_page_uuid: active_page,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeTab(idx) => {
                let page = self.pages[idx].uniq_id;
                self.active_page_uuid = page;
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.pages = props.pages;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-tab">
                <nav class="jinya-tab__link-list">
                    {for self.pages.iter().enumerate().map(move |(idx, item)| {
                        html! {
                            <a class=self.get_link_class(item.uniq_id) onclick=self.link.callback(move |_| Msg::ChangeTab(idx))>{&item.title}</a>
                        }
                    })}
                </nav>
                <div class="jinya-tab__pages">
                    {for self.pages.iter().enumerate().map(|(_, mut item)| {
                        html! {
                            <div class=self.get_page_class(item.uniq_id)>
                                {item.content.clone()}
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }
}

impl TabControl {
    fn get_page_class(&self, id: Uuid) -> String {
        if id == self.active_page_uuid {
            "jinya-tab__page jinya-tab__page--active".to_string()
        } else {
            "jinya-tab__page".to_string()
        }
    }

    fn get_link_class(&self, id: Uuid) -> String {
        if id == self.active_page_uuid {
            "jinya-tab__link jinya-tab__link--active".to_string()
        } else {
            "jinya-tab__link".to_string()
        }
    }
}
