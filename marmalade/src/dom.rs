use std::clone::Clone;

thread_local! {
    static WINDOW: web_sys::Window = web_sys::window().unwrap();
    static DOCUMENT: web_sys::Document = WINDOW.with(web_sys::Window::document).unwrap();
    static PERFORMANCE: web_sys::Performance = WINDOW.with(web_sys::Window::performance).unwrap();
    static BODY: web_sys::HtmlElement = DOCUMENT.with(web_sys::Document::body).unwrap();
}

#[must_use]
pub fn window() -> web_sys::Window {
    WINDOW.with(Clone::clone)
}

#[must_use]
pub fn document() -> web_sys::Document {
    DOCUMENT.with(Clone::clone)
}

#[must_use]
pub fn performance() -> web_sys::Performance {
    PERFORMANCE.with(Clone::clone)
}

#[must_use]
pub fn body() -> web_sys::HtmlElement {
    BODY.with(Clone::clone)
}
