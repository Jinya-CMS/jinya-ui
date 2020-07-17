use yew::Children;
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-table {
    width: 100%;
    border-spacing: 0;
    border-collapse: collapse;
}

.jinya-table__column-header {
    font-weight: bold;
    border-bottom: 2px solid var(--primary-color);
    text-align: left;
    color: var(--primary-color);
    padding: 0.75rem 0.75rem 0.5rem;
}

.jinya-table__body tr {
    color: var(--primary-color);
    cursor: pointer;
    background: var(--white);
    transition: all 0.3s;
}

.jinya-table__body tr:nth-child(2n + 1) {
    background: var(--input-background-color);
}

.jinya-table__body tr:hover {
    background: var(--disabled-color);
}

.jinya-table__body tr.jinya-table__row--selected {
    background: var(--primary-color);
    color: var(--white);
}

.jinya-table__body td {
    padding: 0.75rem;
}
"
}

pub struct Table {
    headers: Vec<String>,
    children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableProps {
    pub headers: Vec<String>,
    pub children: Children,
}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Table {
            headers: props.headers,
            children: props.children,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.children = _props.children;
        self.headers = _props.headers;

        true
    }

    fn view(&self) -> Html {
        html! {
            <table class="jinya-table">
                <thead>
                    <tr>
                        {for self.headers.iter().enumerate().map(|(_, item)| {
                            html! {
                                <th class="jinya-table__column-header">{&item}</th>
                            }
                        })}
                    </tr>
                </thead>
                <tbody class="jinya-table__body">
                    {for self.children.iter().enumerate().map(|(_, child)| {
                        child
                    })}
                </tbody>
            </table>
        }
    }
}