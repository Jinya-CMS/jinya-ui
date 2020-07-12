use yew::Callback;
use yew::prelude::*;

use crate::dialog::utils::dialog::*;

pub type DialogType = super::utils::dialog::DialogType;

pub struct ConfirmationDialog {
    title: String,
    message: String,
    approve_label: String,
    decline_label: String,
    is_open: bool,
    on_approve: Callback<()>,
    on_decline: Callback<()>,
    dialog_type: DialogType,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ConfirmationDialogProps {
    pub title: String,
    pub message: String,
    pub approve_label: String,
    pub decline_label: String,
    pub is_open: bool,
    pub on_approve: Callback<()>,
    pub on_decline: Callback<()>,
    #[prop_or(DialogType::Primary)]
    pub dialog_type: DialogType,
}

impl Component for ConfirmationDialog {
    type Message = Msg;
    type Properties = ConfirmationDialogProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ConfirmationDialog {
            is_open: props.is_open,
            title: props.title,
            message: props.message,
            approve_label: props.approve_label,
            decline_label: props.decline_label,
            on_approve: props.on_approve,
            on_decline: props.on_decline,
            dialog_type: props.dialog_type,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.is_open = _props.is_open;
        self.on_decline = _props.on_decline;
        self.on_approve = _props.on_approve;
        self.approve_label = _props.approve_label;
        self.decline_label = _props.decline_label;
        self.message = _props.message;
        self.title = _props.title;
        self.dialog_type = _props.dialog_type;

        true
    }

    fn view(&self) -> Html {
        html! {
            <Dialog is_open=&self.is_open dialog_type=&self.dialog_type title=&self.title on_primary=&self.on_approve on_secondary=&self.on_decline primary_label=&self.approve_label secondary_label=&self.decline_label>
                <p>{&self.message}</p>
            </Dialog>
        }
    }
}
