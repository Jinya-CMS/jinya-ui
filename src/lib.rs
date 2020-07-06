#![recursion_limit = "10240"]

use web_sys::window;

use crate::layout::*;
use crate::widgets::*;

mod id_generator;
pub mod widgets;
pub mod layout;

pub fn init() {
    // language=CSS
    let root_css = "
@import url(\"https://use.typekit.net/jmo2xoh.css\");

:root {
    font-size: 16px;

    /* Colors: */
    --white: #FFFFFF;
    --secondary-color: #966554;
    --negative-color: #A61C13;
    --positive-color: #146621;
    --primary-color: #514B57;
    --information-color: #182B70;
    --disabled-border-color: #A8A1AE;
    --disabled-color: #D3D0D7;
    --input-background-color: #F4F3F5;
    --dropback: #DBDBDB;
    --menu-bar-box-shadown: #00000029;

    /* Font/text values */
    --font-family: objektiv-mk1, sans-serif;
    --font-style-regular: 400;
    --font-style-bold: 700;
    --font-style-light: 300;
    --font-size-12: 12px;
    --font-size-16: 16px;
    --font-size-24: 24px;
    --font-size-32: 32px;

    --line-height-18: 18px;
    --line-height-23: 23px;
    --line-height-34: 34px;
    --line-height-46: 46px;

    font-family: var(--font-family);
    font-weight: var(--font-style-regular);
    line-height: var(--line-height-23);
}

html,
body {
    padding: 0;
    margin: 0;
    color: var(--primary-color);
    background: var(--white);
}

@media (prefers-color-scheme: dark) {
    :root {
        --white: #000000;
        --menu-bar-box-shadown: #FFFFFF29;
        --input-background-color: #0B0A0C;
    }
}

h1, h2, h3, h4, h5, h6 {
    text-decoration: underline;
    margin-bottom: 0;
}

h1 {
    font-size: var(--font-size-32);
    line-height: var(--line-height-46);
}

h2 {
    font-size: var(--font-size-24);
    line-height: var(--line-height-34);
}

h3 {
    font-size: var(--font-size-16);
    line-height: var(--line-height-23);
}
";
    let css = vec![
        root_css,
        button::get_css(),
        button_row::get_css(),
        page::get_css(),
        menu::bar::get_css(),
        menu::item::get_css(),
        form::input::get_css(),
        form::checkbox::get_css(),
        form::radio::get_css(),
    ];
    let doc = window().unwrap().document().unwrap();
    let style_element = doc.create_element("style").unwrap();
    style_element.set_text_content(Some(&css.join("\n")));
    doc.query_selector("head").unwrap().expect("Head not found").append_child(&style_element);
}