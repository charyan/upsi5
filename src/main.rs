use entity::Ball;
use entity::BallType;
use glam::Mat3;
use glam::Vec2;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::image;
use marmalade::input;
use marmalade::input::Button;
use marmalade::input::Key;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::canvas2d::TextureRect;
use marmalade::render::color;
use marmalade::tick_scheduler::TickScheduler;
use std::collections::BTreeMap;
use std::mem;
use std::time::Duration;
use world::WORLD_DIM;
use world::World;

mod entity;
mod upgrade;
mod world;

const BORDER_SIZE: f32 = 0.068;

fn game_tick(game: &mut Game) {
    if game.state == GameState::Running {
        if !game.world.tick() {
            game.state = GameState::Playing;
            game.world.spawn_round();
        }
    }
}

struct Resources {
    pool_table: TextureRect,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameState {
    Running,
    Playing,
}

struct Game {
    world: World,
    state: GameState,
    moves: BTreeMap<usize, Vec2>,
    selected: Option<usize>,
}

fn draw_line(canvas: &mut Canvas2d, position: Vec2, length: Vec2, width: f32) {
    let angle = (-length).to_angle();

    let line = length.length();

    let previous = canvas.get_view_matrix();

    let m1 = Mat3::from_translation(position);
    let m2 = Mat3::from_rotation_z(angle);
    let m3 = Mat3::from_translation(-position);

    canvas.set_view_matrix(previous * m1 * m2 * m3);

    canvas.draw_rect(
        position - Vec2::new(0., width / 2.),
        Vec2::new(line, width),
        color::WHITE,
        &canvas.white_texture(),
    );

    canvas.set_view_matrix(previous);
}

fn render_tick(canvas: &mut Canvas2d, game: &mut Game, resources: &Resources) {
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

    for ball in &game.world.balls {
        canvas.draw_regular(
            ball.borrow().position,
            ball.borrow().radius,
            64,
            if let BallType::Enemy(_) = ball.borrow().letypedelaboule { color::rgb(1., 0.5, 0.5)} else {color::rgb(0.098, 0.76, 0.01)} ,
            &canvas.white_texture(),
        );
    }

    if game.state == GameState::Playing {
        for (i, b) in game.world.balls.iter().enumerate() {
            if let Some(&m) = game.moves.get(&i) {
                let b = b.borrow();

                draw_line(canvas, b.position, m / 0.01, 0.005);
            }
        }

        if input::is_button_pressed(Button::Left) {
            let mouse_pos = input::mouse_position().as_vec2();

            for (i, b) in game.world.balls.iter().enumerate() {
                let b = b.borrow();

                if let BallType::Player = b.letypedelaboule {
                    if b.position.distance(canvas.screen_to_world_pos(mouse_pos)) < b.radius {
                        game.selected = Some(i);
                    }
                }
            }
        }
        if let Some(selected) = game.selected {
            let move_vector = (canvas.screen_to_world_pos(input::mouse_position().as_vec2())
                - game.world.balls[selected].borrow().position)
                * 0.01;

            if !input::is_button_down(Button::Left) {
                game.moves.insert(selected, move_vector);
                game.selected = None;
            }

            draw_line(
                canvas,
                game.world.balls[selected].borrow().position,
                move_vector / 0.01,
                0.01,
            );
        }
        if input::is_key_pressed(Key::Space) {
            game.state = GameState::Running;

            let moves = mem::replace(&mut game.moves, BTreeMap::new());

            game.world.launch_round(moves);
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

    let mut game = Game {
        moves: BTreeMap::new(),
        world: World::new(),
        state: GameState::Playing,
        selected: None,
    };
    game.world.spawn_round();

    let mut tick_scheduler = TickScheduler::new(Duration::from_millis(1));

    draw_scheduler::set_on_draw(move || {
        for _ in 0..tick_scheduler.tick_count() {
            game_tick(&mut game);
        }

        render_tick(&mut canvas, &mut game, &resources);
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
