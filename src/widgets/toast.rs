use gloo_timers::callback::Timeout;
use web_sys::Element;
use yew::utils::document;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-toast__container {
    display: flex;
    max-width: 20rem;
    width: 20rem;
    right: 0;
    top: 4rem;
    position: fixed;
    gap: 1rem;
    flex-flow: row wrap;
}

.jinya-toast {
    border-radius: 5px;
    border: 2px solid var(--state-color);
    margin-right: 1rem;
    flex: 1 1 auto;
    display: flex;
    position: relative;
    padding: 0.5rem 0.5rem 0.5rem 2.75rem;
    left: 20rem;
    transition: left 0.3s;
    background: var(--white);
}

.jinya-toast::before {
    content: '';
    border-radius: 50%;
    background: var(--state-color);
    width: 0.75rem;
    height: 0.75rem;
    position: absolute;
    top: 50%;
    left: 1rem;
    transform: translateY(-50%);
}

.jinya-toast--open {
    left: 0;
}

.jinya-toast--primary {
    --state-color: var(--primary-color);
}

.jinya-toast--positive {
    --state-color: var(--positive-color);
}

.jinya-toast--negative {
    --state-color: var(--negative-color);
}

.jinya-toast--information {
    --state-color: var(--information-color);
}
"
}

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
        Timeout::new(1, move || {
            toast.class_list().add_1("jinya-toast--open").unwrap();

            Timeout::new(5_000, move || {
                toast.remove();
            }).forget();
        }).forget();

    }
}

pub struct Toast;

impl Toast {
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
}
