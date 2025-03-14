use crate::dom::{body, document};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, Node};

const FULL_SCREEN_CANVAS_CSS: &str = "position:absolute;top:0;left:0;";

pub fn set_title(title: &str) {
    document().set_title(title);
}

#[must_use]
pub fn create_full_screen_canvas() -> HtmlCanvasElement {
    let canvas = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas.style().set_css_text(FULL_SCREEN_CANVAS_CSS);

    canvas
}

pub fn stack_node<T: AsRef<Node>>(node: T) {
    body().append_child(node.as_ref()).unwrap();
}

pub fn pop_node() {
    let body = body();

    body.remove_child(&body.last_child().unwrap()).unwrap();
}
