use web_sys::console;

pub fn log(msg: &str) {
    console::log_1(&msg.into());
}

pub fn info(msg: &str) {
    console::info_1(&msg.into());
}

pub fn warn(msg: &str) {
    console::warn_1(&msg.into());
}

pub fn error(msg: &str) {
    console::error_1(&msg.into());
}
