use yew::prelude::*;
use yew::{Component, ComponentLink, Html};
use yew_router::{prelude::*, Switch};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-menu__item {
    padding: 1rem;
    display: inline-block;
    cursor: pointer;
    position: relative;
}

.jinya-menu__flyout {
    background: var(--primary-color);
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 5px;
    display: none;
    cursor: default;
    position: absolute;
    grid-auto-columns: auto;
    grid-auto-flow: column;
    grid-gap: 1rem;
    transform: translateX(-50%);
    left: 50%;
    z-index: 2;
}

.jinya-menu__flyout::before {
    content: '';
    border: 1rem solid transparent;
    border-bottom-color: var(--primary-color);
    position: absolute;
    top: -1rem;
    left: 50%;
    transform: translateX(-50%);
    border-bottom-width: 0.5rem;
    border-top-width: 0.5rem;
}

.jinya-menu__item:hover > .jinya-menu__flyout {
    display: grid;
}

.jinya-menu__subitem {
    font-size: var(--font-size-16);
    color: var(--white);
    font-weight: var(--font-weight-light);
    cursor: pointer;
    text-decoration: none;
    white-space: pre;
}

.jinya-menu__group {
    display: flex;
    flex-flow: column wrap;
    align-items: center;
}

.jinya-menu__group-header {
    font-size: var(--font-size-24);
    color: var(--white);
    font-weight: var(--font-style-light);
    margin-bottom: 0.5rem;
    white-space: pre;
}
"
}

#[derive(Clone, PartialEq, Properties)]
pub struct SubItem<RouteType: Switch + Clone + 'static> {
    pub label: String,
    #[prop_or_default]
    pub route: Option<&'static RouteType>,
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SubItemGroup<RouteType: Switch + Clone + 'static> {
    pub title: String,
    pub items: Vec<SubItem<RouteType>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuItem<RouteType: Switch + Clone + 'static> {
    pub label: String,
    pub groups: Vec<SubItemGroup<RouteType>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuItemProps<RouteType: Switch + Clone + 'static> {
    pub groups: Vec<SubItemGroup<RouteType>>,
    pub label: String,
}

impl<RouteType: Switch + Clone + 'static> Component for MenuItem<RouteType> {
    type Message = ();
    type Properties = MenuItemProps<RouteType>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MenuItem {
            groups: props.groups,
            label: props.label,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.groups = _props.groups;
        self.label = _props.label;
        true
    }

    fn view(&self) -> Html {
        html! {
            <li href="#" class="jinya-menu__item">
                {&self.label}
                <div class="jinya-menu__flyout">
                    {for self.groups.iter().enumerate().map(|(_, mut group)| {
                        html! {
                            <div class="jinya-menu__group">
                                <span class="jinya-menu__group-header">{&group.title}</span>
                                {for group.items.iter().enumerate().map(|(_, mut subitem)| {
                                    if subitem.route.is_some() {
                                        html! {
                                            <RouterAnchor<RouteType> classes="jinya-menu__subitem" route=subitem.route.unwrap()>{&subitem.label}</RouterAnchor<RouteType>>
                                        }
                                    } else {
                                        html! {
                                            <a href="#" class="jinya-menu__subitem" onclick=subitem.on_click.as_ref().unwrap()>{&subitem.label}</a>
                                        }
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            </li>
        }
    }
}
