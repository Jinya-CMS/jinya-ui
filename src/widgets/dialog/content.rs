use yew::Callback;
use yew::prelude::*;

use crate::widgets::dialog::utils::dialog::*;

pub struct ContentDialog {
    title: String,
    children: Children,
    primary_label: String,
    secondary_label: String,
    is_open: bool,
    on_primary: Callback<()>,
    on_secondary: Callback<()>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ContentDialogProps {
    pub title: String,
    pub children: Children,
    pub primary_label: String,
    pub secondary_label: String,
    pub is_open: bool,
    pub on_primary: Callback<()>,
    pub on_secondary: Callback<()>,
}

impl Component for ContentDialog {
    type Message = Msg;
    type Properties = ContentDialogProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ContentDialog {
            is_open: props.is_open,
            title: props.title,
            children: props.children,
            primary_label: props.primary_label,
            secondary_label: props.secondary_label,
            on_primary: props.on_primary,
            on_secondary: props.on_secondary,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.is_open = _props.is_open;
        self.on_secondary = _props.on_secondary;
        self.on_primary = _props.on_primary;
        self.primary_label = _props.primary_label;
        self.secondary_label = _props.secondary_label;
        self.children = _props.children;
        self.title = _props.title;

        true
    }

    fn view(&self) -> Html {
        html! {
            <Dialog children=&self.children is_open=&self.is_open dialog_type=DialogType::Primary title=&self.title on_primary=&self.on_primary on_secondary=&self.on_secondary primary_label=&self.primary_label secondary_label=&self.secondary_label>
            </Dialog>
        }
    }
}
