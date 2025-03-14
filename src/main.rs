use glam::Vec2;
use marmalade::console;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::image;
use marmalade::input;
use marmalade::input::Button;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::canvas2d::TextureRect;
use marmalade::render::color;
use marmalade::tick_scheduler::TickScheduler;
use std::time::Duration;
use world::WORLD_DIM;
use world::World;

mod entity;
mod world;

fn game_tick(world: &mut World) {
    world.tick();
}

struct Resources {
    pool_table: TextureRect,
}

fn render_tick(canvas: &mut Canvas2d, world: &World, resources: &Resources) {
    canvas.fit_screen();

    canvas.pixel_perfect_view();

    canvas.clear(color::rgb(0., 0., 0.));

    canvas.camera_view(WORLD_DIM / 2., WORLD_DIM.x / 2.);

    canvas.draw_rect(
        Vec2::new(0., 0.),
        WORLD_DIM,
        color::WHITE,
        &resources.pool_table,
    );

    for ball in &world.balls {
        canvas.draw_regular(
            ball.borrow().position,
            ball.borrow().radius,
            64,
            color::rgb(1., 0.5, 0.5),
            &canvas.white_texture(),
        );
    }

    if input::is_button_down(Button::Left) {
        let mouse_pos = input::mouse_position().as_vec2();

        for b in &world.balls {
            if b.borrow()
                .position
                .distance(canvas.screen_to_world_pos(mouse_pos))
                < b.borrow().radius
            {
                canvas.draw_regular(
                    b.borrow().position,
                    b.borrow().radius,
                    64,
                    color::rgb(1., 1., 1.),
                    &canvas.white_texture(),
                );
            }
        }
    }

    canvas.flush();
}

async fn async_main() {
    dom_stack::set_title("Slime Pool");

    let main_canvas = dom_stack::create_full_screen_canvas();

    dom_stack::stack_node(&main_canvas);

    let mut canvas = Canvas2d::new(&main_canvas);

    let pool_table = image::from_bytes(include_bytes!("../assets/pool_table.png")).await;

    let pool_table = canvas.create_texture(&pool_table);

    let resources = Resources { pool_table };

    let mut world = World::new();

    world.add_ball(Vec2::new(0.2, 0.5), 0.010, 1., 0.999);

    world.balls[0].borrow_mut().speed = Vec2::new(0.0005, 0.0005);

    let mut tick_scheduler = TickScheduler::new(Duration::from_millis(1));

    draw_scheduler::set_on_draw(move || {
        for _ in 0..tick_scheduler.tick_count() {
            game_tick(&mut world);
        }

        render_tick(&mut canvas, &world, &resources);
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
