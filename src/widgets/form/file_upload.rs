use yew::prelude::*;
use yew::services::reader::File;
use yew::{Callback, Component, ComponentLink, Html};

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-file-upload__color-container--default {
    --state-color: var(--primary-color);
}

.jinya-file-upload__color-container--negative {
    --state-color: var(--negative-color);
}

.jinya-file-upload__color-container--positive {
    --state-color: var(--positive-color);
}

.jinya-file-upload__color-container--disabled {
    --state-color: var(--disabled-border-color);
}

.jinya-file-upload__color-container--drag-over .jinya-file-upload__label,
.jinya-file-upload__color-container--drag-over .jinya-file-upload__file-info,
.jinya-file-upload__color-container--drag-over .jinya-file-upload__container {
    background-color: var(--input-background-color);
}

.jinya-file-upload__color-container {
    display: flex;
    position: relative;
    width: 100%;
    flex: 0 0 100%;
    flex-flow: row wrap;
}

.jinya-file-upload__container {
    display: flex;
    border: 2px solid var(--state-color);
    border-radius: 5px;
    padding: 0.5rem 0.75rem 0.25rem;
    position: relative;
    margin-top: 0.75rem;
    width: 100%;
    box-sizing: border-box;
}

.jinya-file-upload__input {
    display: none;
}

.jinya-file-upload__file-info {
    font-size: var(--font-size-16);
    color: var(--state-color);
    background: var(--white);
    border: none;
    padding: 0;
    width: 100%;
}

.jinya-file-upload__file-info:disabled {
    cursor: not-allowed;
}

.jinya-file-upload__label {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
    position: absolute;
    top: -0.75rem;
    background: var(--white);
    padding-left: 0.25rem;
    padding-right: 0.25rem;
    box-sizing: border-box;
    left: 0.5rem;
    z-index: 0;
}

.jinya-file-upload__validation-message {
    display: block;
    font-size: var(--font-size-12);
    color: var(--state-color);
}

.jinya-file-upload__button {
    margin-left: auto;
    font-size: var(--font-size-24);
    border: 2px solid var(--state-color);
    height: calc(100% - 2rem);
    top: 0.75rem;
    position: absolute;
    right: 0;
    padding: 0.5rem;
    transition: color 0.3s, background 0.3s;
    background: var(--white);
    border-bottom-right-radius: 5px;
    border-top-right-radius: 5px;
    cursor: pointer;
}

.jinya-file-upload__button:hover {
    color: var(--white);
    background: var(--state-color);
}
"
}

#[derive(Clone, PartialEq)]
pub enum FileUploadState {
    Default,
    Negative,
    Positive,
}

pub struct FileUpload {
    link: ComponentLink<Self>,
    label: String,
    state: FileUploadState,
    validation_message: String,
    placeholder: String,
    filename: String,
    disabled: bool,
    on_select: Callback<Vec<File>>,
    is_drag_over: bool,
    multiple: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FileUploadProps {
    pub label: String,
    #[prop_or(FileUploadState::Default)]
    pub state: FileUploadState,
    #[prop_or("".to_string())]
    pub validation_message: String,
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(false)]
    pub multiple: bool,
    pub on_select: Callback<Vec<File>>,
    #[prop_or("".to_string())]
    pub filename: String,
}

pub enum Msg {
    Files(Vec<File>),
    Drop(DragEvent),
    DragOver(DragEvent),
    DragExit,
}

impl Default for FileUploadState {
    fn default() -> Self {
        FileUploadState::Default
    }
}

impl FileUpload {
    fn get_input_container_class(&self) -> String {
        let class = if self.disabled {
            "jinya-file-upload__color-container jinya-file-upload__color-container--disabled"
                .to_string()
        } else {
            match self.state {
                FileUploadState::Default => "jinya-file-upload__color-container jinya-file-upload__color-container--default",
                FileUploadState::Negative => "jinya-file-upload__color-container jinya-file-upload__color-container--negative",
                FileUploadState::Positive => "jinya-file-upload__color-container jinya-file-upload__color-container--positive",
            }.to_string()
        };

        if self.is_drag_over {
            format!("{} jinya-file-upload__color-container--drag-over", class)
        } else {
            class
        }
    }
}

impl Component for FileUpload {
    type Message = Msg;
    type Properties = FileUploadProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FileUpload {
            link,
            label: props.label,
            state: props.state,
            validation_message: props.validation_message,
            placeholder: props.placeholder,
            disabled: props.disabled,
            on_select: props.on_select,
            filename: props.filename,
            is_drag_over: false,
            multiple: props.multiple,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(value) => {
                self.on_select.emit(value.clone());
                if value.len() > 0 {
                    self.filename = if self.multiple {
                        let name = value
                            .iter()
                            .map(|file| file.name())
                            .collect::<Vec<String>>()
                            .join(", ");
                        name
                    } else {
                        value.first().unwrap().name()
                    }
                }
            }
            Msg::Drop(event) => {
                self.is_drag_over = false;
                event.prevent_default();
                event.stop_propagation();
                let data_transfer = event.data_transfer().unwrap();
                let files: Vec<File> = js_sys::try_iter(&data_transfer.files().unwrap())
                    .unwrap()
                    .unwrap()
                    .map(|v| File::from(v.unwrap()))
                    .collect();
                if files.len() > 0 {
                    self.on_select.emit(files.clone());
                    self.filename = if self.multiple {
                        let name = files
                            .iter()
                            .map(|file| file.name())
                            .collect::<Vec<String>>()
                            .join(", ");
                        name
                    } else {
                        files.first().unwrap().name()
                    }
                }
            }
            Msg::DragOver(event) => {
                self.is_drag_over = true;
                event.prevent_default();
                event.stop_propagation();
                let data_transfer = event.data_transfer().unwrap();
                data_transfer.set_drop_effect("copy");
            }
            Msg::DragExit => {
                self.is_drag_over = false;
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.label = _props.label;
        self.state = _props.state;
        self.validation_message = _props.validation_message;
        self.placeholder = _props.placeholder;
        self.disabled = _props.disabled;

        true
    }

    fn view(&self) -> Html {
        let id = super::super::super::id_generator::generate_id();
        html! {
            <div
                class=self.get_input_container_class()
                ondrop=self.link.callback(move |event| {
                    Msg::Drop(event)
                })
                ondragover=self.link.callback(move |event: DragEvent| {
                    Msg::DragOver(event)
                })
                ondragexit=self.link.callback(|_| {
                    Msg::DragExit
                })
            >
                <div class="jinya-file-upload__color-container">
                    <div class="jinya-file-upload__container">
                        <label for=id class="jinya-file-upload__label">{&self.label}</label>
                        <input
                            id=id
                            type="file"
                            disabled=self.disabled
                            placeholder=self.placeholder
                            class="jinya-file-upload__input"
                            multiple=self.multiple
                            onchange=self.link.callback(move |value| {
                                let mut result = vec![];
                                if let ChangeData::Files(files) = value {
                                    let files = js_sys::try_iter(&files)
                                        .unwrap()
                                        .unwrap()
                                        .into_iter()
                                        .map(|v| File::from(v.unwrap()));
                                    result.extend(files);
                                }
                                Msg::Files(result)
                            })
                        />
                        <label for=id class="jinya-file-upload__file-info">
                            {if self.filename.is_empty() {
                                &self.placeholder
                            } else {
                                &self.filename
                            }}
                        </label>
                    </div>
                    <label for=id class="jinya-file-upload__button">
                        <span class="mdi mdi-upload"></span>
                    </label>
                </div>
                <span class="jinya-file-upload__validation-message">{&self.validation_message}</span>
            </div>
        }
    }
}
