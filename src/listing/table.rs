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

.jinya-table__row {
    color: var(--primary-color);
    cursor: pointer;
    background: var(--white);
    transition: all 0.3s;
}

.jinya-table__row:nth-child(2n + 1) {
    background: var(--input-background-color);
}

.jinya-table__row:hover {
    background: var(--disabled-color);
}

.jinya-table__row--selected {
    background: var(--primary-color) !important;
    color: var(--white) !important;
}

.jinya-table__cell {
    padding: 0.75rem;
}
"
}

#[derive(Clone, PartialEq)]
pub struct TableHeader {
    pub title: String,
    pub key: String,
}

pub enum Msg {
    Select(usize),
}

#[derive(Clone, PartialEq)]
pub struct TableCell {
    pub key: String,
    pub value: String,
}

#[derive(Clone, PartialEq)]
pub struct TableRow {
    cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> TableRow {
        TableRow {
            cells,
        }
    }

    pub fn remove_cell(&mut self, key: &str) {
        let idx = self.cells.binary_search_by(|item| item.key.cmp(&key.to_string()));
        if idx.is_ok() {
            self.cells.remove(idx.unwrap());
        }
    }

    pub fn add_cells(&mut self, cells: &mut Vec<TableCell>) {
        self.cells.append(cells);
    }
}

pub struct Table {
    link: ComponentLink<Self>,
    headers: Vec<TableHeader>,
    rows: Vec<TableRow>,
    on_select: Callback<usize>,
    selected_index: Option<usize>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableProps {
    pub headers: Vec<TableHeader>,
    pub rows: Vec<TableRow>,
    #[prop_or_default]
    pub on_select: Callback<usize>,
    #[prop_or(None)]
    pub selected_index: Option<usize>,
}

impl Component for Table {
    type Message = Msg;
    type Properties = TableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Table {
            link,
            headers: props.headers,
            rows: props.rows,
            on_select: props.on_select,
            selected_index: props.selected_index,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(idx) => {
                self.on_select.emit(idx)
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.rows = _props.rows;
        self.headers = _props.headers;
        self.selected_index = _props.selected_index;
        self.on_select = _props.on_select;

        true
    }

    fn view(&self) -> Html {
        html! {
            <table class="jinya-table">
                <thead>
                    <tr>
                        {for self.headers.iter().enumerate().map(|(_, header)| {
                            html! {
                                <th class="jinya-table__column-header">{&header.title}</th>
                            }
                        })}
                    </tr>
                </thead>
                <tbody class="jinya-table__body">
                    {for self.rows.iter().enumerate().map(|(idx, row)| {
                        html! {
                            <tr class=self.get_tr_class(idx) onclick=self.link.callback(move |_| Msg::Select(idx))>
                                {for self.headers.iter().enumerate().map(move |(_, header)| {
                                    html! {
                                        <td class="jinya-table__cell">{self.get_cell_value(row, header)}</td>
                                    }
                                })}
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        }
    }
}

impl Table {
    fn get_cell_value(&self, row: &TableRow, header: &TableHeader) -> String {
        for cell in &row.cells {
            if cell.key.eq(&header.key) {
                return cell.value.to_string();
            }
        }

        return "".to_string();
    }

    fn get_tr_class(&self, idx: usize) -> String {
        if self.selected_index.is_some() && idx == self.selected_index.unwrap() {
            "jinya-table__row jinya-table__row--selected".to_string()
        } else {
            "jinya-table__row".to_string()
        }
    }
}