use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-alert {
    border: 2px solid var(--state-color);
    border-top: 0.5rem solid var(--state-color);
    padding: 0.5rem 1rem;
    border-radius: 5px;
    flex: 0 0 100%;
    box-sizing: border-box;
}

.jinya-alert--primary {
    --state-color: var(--primary-color);
}

.jinya-alert--negative {
    --state-color: var(--negative-color);
}

.jinya-alert--positive {
    --state-color: var(--positive-color);
}

.jinya-alert--information {
    --state-color: var(--information-color);
}

.jinya-alert__message {
    color: var(--state-color);
}

.jinya-alert__message--one-line {
    font-size: var(--font-size-16);
}

.jinya-alert__message--multi-line {
    font-size: var(--font-size-12);
}

.jinya-alert__title {
    font-size: var(--font-size-16);
    display: block;
    width: 100%;
    border-bottom: 2px solid var(--state-color);
}
"
}

#[derive(Clone, PartialEq)]
pub enum AlertType {
    Negative,
    Positive,
    Primary,
    Information,
}

pub struct Alert {
    title: String,
    message: String,
    alert_type: AlertType,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or("".to_string())]
    pub title: String,
    pub message: String,
    pub alert_type: AlertType,
}

impl Component for Alert {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Alert {
            title: props.title,
            message: props.message,
            alert_type: props.alert_type,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.title = _props.title;
        self.message = _props.message;
        self.alert_type = _props.alert_type;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.get_alert_class()>
                {if self.title.is_empty() {
                    html! {
                        <span class="jinya-alert__message jinya-alert__message--one-line">{&self.message}</span>
                    }
                } else {
                    html! {
                        <>
                            <span class="jinya-alert__title">{&self.title}</span>
                            <span class="jinya-alert__message jinya-alert__message--multi-line">{&self.message}</span>
                        </>
                    }
                }}
            </div>
        }
    }
}

impl Alert {
    fn get_alert_class(&self) -> String {
        match self.alert_type {
            AlertType::Primary => "jinya-alert jinya-alert--primary",
            AlertType::Negative => "jinya-alert jinya-alert--negative",
            AlertType::Positive => "jinya-alert jinya-alert--positive",
            AlertType::Information => "jinya-alert jinya-alert--information",
        }.to_string()
    }
}