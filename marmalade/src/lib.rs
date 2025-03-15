#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::future_not_send,
    clippy::cast_possible_truncation,
    clippy::missing_panics_doc,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::new_without_default,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names,
    clippy::suboptimal_flops,
    clippy::too_many_lines
)]

pub mod audio;
pub mod console;
pub mod dom;
pub mod dom_stack;
pub mod draw_scheduler;
pub mod font;
pub mod image;
pub mod input;
pub mod net;
pub mod rand;
pub mod render;
pub mod tick_scheduler;
pub mod time;
