use web_sys::Element;
use yew::utils::document;

pub fn toast_container() -> Element {
    let doc = document();
    let container = doc.create_element("div").unwrap();
    container.set_class_name("jinya-toast__container");

    container
}

enum ToastType {
    Primary,
    Positive,
    Negative,
    Information,
}

fn toast(message: String, toast_type: ToastType) {
    let container = document().query_selector(".jinya-toast__container");
    if container.is_ok() {
        let toast = document().create_element("div").unwrap();
        toast.set_inner_html(message.as_str());
        toast.class_list().add_1("jinya-toast").unwrap();

        toast.class_list().add_1(match toast_type {
            ToastType::Primary => "jinya-toast--primary",
            ToastType::Positive => "jinya-toast--positive",
            ToastType::Negative => "jinya-toast--negative",
            ToastType::Information => "jinya-toast--information",
        }).unwrap();

        container.unwrap().unwrap().append_child(&toast).unwrap();
        toast.class_list().add_1("jinya-toast--open").unwrap();
    }
}

pub fn primary_toast(message: String) {
    toast(message, ToastType::Primary)
}

pub fn positive_toast(message: String) {
    toast(message, ToastType::Positive)
}

pub fn negative_toast(message: String) {
    toast(message, ToastType::Negative)
}

pub fn information_toast(message: String) {
    toast(message, ToastType::Information)
}