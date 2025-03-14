use glam::Vec2;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::font;
use marmalade::font::Font;
use marmalade::input;
use marmalade::input::Key;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::color;
use marmalade::tick_scheduler::TickScheduler;
use std::cell::RefCell;
use std::time::Duration;

mod entity;
mod world;

fn game_tick(position: &mut Vec2) {
    if input::is_key_down(Key::A) {
        position.x -= 0.5;
    }
    if input::is_key_down(Key::D) {
        position.x += 0.5;
    }
    if input::is_key_down(Key::S) {
        position.y -= 0.5;
    }
    if input::is_key_down(Key::W) {
        position.y += 0.5;
    }
}

fn render_tick(canvas: &mut Canvas2d, font: &mut Font, position: Vec2) {
    canvas.fit_screen();

    canvas.pixel_perfect_view();

    canvas.clear(color::rgb(0., 0., 0.));

    canvas.draw_regular(
        position,
        100.,
        64,
        color::rgb(1., 0.5, 0.5),
        &canvas.white_texture(),
    );

    canvas.draw_text(
        Vec2::new(100., 100.),
        50.,
        "Welcome to the pool",
        font,
        color::rgb(1., 1., 1.),
        &canvas.white_texture(),
    );

    canvas.flush();
}

async fn async_main() {
    let a = RefCell::new(String::new());

    dom_stack::set_title("Slime Pool");

    let main_canvas = dom_stack::create_full_screen_canvas();

    dom_stack::stack_node(&main_canvas);

    let mut canvas = Canvas2d::new(&main_canvas);

    let mut font = font::from_bytes(font::MONOGRAM);

    let mut position = Vec2::new(300., 300.);

    let mut tick_scheduler = TickScheduler::new(Duration::from_millis(1));

    draw_scheduler::set_on_draw(move || {
        for _ in 0..tick_scheduler.tick_count() {
            game_tick(&mut position);
        }

        render_tick(&mut canvas, &mut font, position);
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
