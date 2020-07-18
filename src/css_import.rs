use web_sys::Element;
use web_sys::Document;

pub(crate) fn get_typekit(document: &Document) -> Element {
    let link_element = document.create_element("link").expect("Element could not be created");
    link_element.set_attribute("href", "https://use.typekit.net/jmo2xoh.css").expect("Was not able to set attribute");
    link_element.set_attribute("rel", "stylesheet").expect("Was not able to set attribute");

    link_element
}

pub(crate) fn get_mdi(document: &Document) -> Element {
    let link_element = document.create_element("link").expect("Element could not be created");
    link_element.set_attribute("href", "https://cdn.materialdesignicons.com/5.3.45/css/materialdesignicons.min.css").expect("Was not able to set attribute");
    link_element.set_attribute("rel", "stylesheet").expect("Was not able to set attribute");

    link_element
}