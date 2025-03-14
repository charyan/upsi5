use glam::Vec2;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::font;
use marmalade::image;
use marmalade::input;
use marmalade::input::Key;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::color;

async fn async_main() {
    // Set the window title
    dom_stack::set_title("Hello World");

    // Create an HtmlCanvas where the game will be displayed
    let main_canvas = dom_stack::create_full_screen_canvas();
    // Add the Html canvas to the dom
    dom_stack::stack_node(&main_canvas);

    // Create a context for drawing the "game"
    let mut canvas = Canvas2d::new(&main_canvas);

    // Load an image
    let image = image::from_bytes(include_bytes!("../../../resources/images/logo.png")).await;

    // Upload the image to the GPU
    let image_rect = canvas.create_texture(&image);

    // Load the default font
    let mut font = font::from_bytes(font::MONOGRAM);

    // Player position
    let mut position = Vec2::new(300., 300.);

    // Closure called for every frame
    draw_scheduler::set_on_draw(move || {
        // Move the sprite with keyboard
        if input::is_key_down(Key::A) {
            position.x -= 4.;
        }
        if input::is_key_down(Key::D) {
            position.x += 4.;
        }
        if input::is_key_down(Key::S) {
            position.y -= 4.;
        }
        if input::is_key_down(Key::W) {
            position.y += 4.;
        }

        // Set size of the canvas to the same as screen
        canvas.fit_screen();

        // Set the view matrix so that coordinates corresponds to pixels on the canvas
        canvas.pixel_perfect_view();

        // Clear canvas to black
        canvas.clear(color::rgb(0., 0., 0.));

        // Create an hexagon with our texture and a red filter then draw it
        canvas.draw_regular(position, 100., 6, color::rgb(1., 0.5, 0.5), &image_rect);

        canvas.draw_text(
            Vec2::new(100., 100.),
            50.,
            "Move with W A S D",
            &mut font,
            color::rgb(1., 1., 1.),
            &canvas.white_texture(),
        );

        // Make sure everything is drawn
        canvas.flush();
    });
}

fn main() {
    // Redirect rust panics to the console for easier debugging
    console_error_panic_hook::set_once();

    // Start the async_main function, some marmalade functionalities require an async context
    wasm_bindgen_futures::spawn_local(async_main());
}
