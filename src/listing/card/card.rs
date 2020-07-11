use yew::Children;
use yew::prelude::*;

use crate::listing::card::*;

pub fn get_css<'a>() -> &'a str {
    //language=CSS
    "
.jinya-card {
    border-radius: 5px;
    min-width: 18rem;
    max-width: 100%;
    position: relative;
    height: 20rem;
    max-height: 20rem;
    display: inline-block;
    transition: all 0.3s;
    flex: 1 1 auto;
    overflow-y: hidden;
}

.jinya-card:hover {
    box-shadow: 0 0 8px var(--menu-bar-box-shadown);
}

.jinya-card:hover .jinya-card__header {
    border-color: var(--white);
}

.jinya-card:hover .jinya-card__button-row {
    opacity: 1;
}

.jinya-card__image {
    width: 100%;
    height: 100%;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
    object-fit: cover;
}
"
}

pub struct Card {
    title: String,
    buttons: Children,
    src: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
   pub title: String,
   pub children: Children,
   pub src: String,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Card {
            title: props.title,
            buttons: props.children,
            src: props.src,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.title = _props.title;
        self.buttons = _props.children;
        self.src = _props.src;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-card">
                <CardHeader title=&self.title />
                <img class="jinya-card__image" src=&self.src alt=&self.title />
                <CardButtonRow>
                    {for self.buttons.iter().enumerate().map(|(_, mut button)| {
                        button
                    })}
                </CardButtonRow>
            </div>
        }
    }
}

