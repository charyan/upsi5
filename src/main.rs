use entity::BallType;
use glam::Mat3;
use glam::Vec2;
use marmalade::audio;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::input;
use marmalade::input::Button;
use marmalade::input::Key;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::canvas2d::TextureRect;
use marmalade::render::color;
use marmalade::tick_scheduler::TickScheduler;
use resources::Resources;
use std::collections::BTreeMap;
use std::mem;
use std::time::Duration;
use world::Sounds;
use world::WORLD_DIM;
use world::World;

mod entity;
mod resources;
mod world;

const BORDER_SIZE: f32 = 0.068;

const PRICE_MAX_SPEED: [u32; 5] = [500, 1500, 3000, 5000, 10000];
const PRICE_START_MASS: [u32; 5] = [500, 1500, 3000, 5000, 10000];
const PRICE_AIM_ASSIST: [u32; 5] = [500, 1500, 3000, 5000, 10000];
const PRICE_PROFITABILITY: [u32; 5] = [500, 1500, 3000, 5000, 10000];
const PRICE_SLIDING: [u32; 5] = [500, 1500, 3000, 5000, 10000];

fn game_tick(game: &mut Game, resources: &mut Resources) {
    if game.state == GameState::Running {
        let (run, sounds) = game.world.tick();
        if !run {
            game.state = GameState::Playing;
            game.world.spawn_round();
        }
        if game.world.is_game_over() {
            game.state = GameState::GameOver;
            game.total_money += game.world.money;
        }
        for sound in sounds {
            match sound {
                Sounds::SlimeSlime => {
                    audio::play(&resources.sounds_slimeslime, 1.);
                }
                Sounds::Coin => {
                    audio::play(&resources.sounds_coin, 1.);
                }
                _ => {}
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameState {
    Running,
    Playing,
    GameOver,
    Shopping,
    Menu,
}

struct Game {
    world: World,
    state: GameState,
    moves: BTreeMap<usize, Vec2>,
    selected: Option<usize>,
    total_money: u32,
    max_speed_level: usize,
    start_mass_level: usize,
    aim_assist_level: usize,
    profitability_level: usize,
    sliding_level: usize,
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

fn draw_ball(canvas: &mut Canvas2d, position: Vec2, radius: f32, texture: &TextureRect) {
    canvas.draw_rect(
        position - Vec2::splat(radius),
        Vec2::splat(radius * 2.),
        color::WHITE,
        texture,
    );
}

fn draw_game(canvas: &mut Canvas2d, game: &mut Game, resources: &mut Resources) {
    let table_size: Vec2 = WORLD_DIM + Vec2::splat(BORDER_SIZE * 2.);

    canvas.camera_view(
        table_size / 2. - Vec2::splat(BORDER_SIZE),
        table_size.x / 2.,
    );

    for &coin in &game.world.coins {
        draw_ball(canvas, coin, world::COIN_RADIUS, &resources.coin);
    }

    for ball in &game.world.balls {
        let ball = ball.borrow();

        draw_ball(
            canvas,
            ball.position,
            ball.radius,
            if let BallType::Enemy(e) = ball.letypedelaboule {
                &resources.balls[e.timer]
            } else {
                &resources.slimeball
            },
        );
    }
}

fn render_tick(canvas: &mut Canvas2d, game: &mut Game, resources: &mut Resources) {
    canvas.fit_screen();

    canvas.clear(color::rgb(0., 0., 0.));

    let table_size: Vec2 = WORLD_DIM + Vec2::splat(BORDER_SIZE * 2.);

    canvas.camera_view(table_size / 2., table_size.x / 2.);

    canvas.draw_rect(
        Vec2::new(0., 0.),
        table_size,
        color::WHITE,
        &resources.pool_table,
    );

    match game.state {
        GameState::Playing => {
            draw_game(canvas, game, resources);
            for (i, b) in game.world.balls.iter().enumerate() {
                if let Some(&m) = game.moves.get(&i) {
                    let b = b.borrow();

                    draw_line(canvas, b.position, m, 0.005);
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
                let mut move_vector = canvas.screen_to_world_pos(input::mouse_position().as_vec2())
                    - game.world.balls[selected].borrow().position;

                if move_vector.length() > 0.15 {
                    move_vector *= 0.15 / move_vector.length();
                }

                if !input::is_button_down(Button::Left) {
                    game.moves.insert(selected, move_vector);
                    game.selected = None;
                }

                let ball_pos = game.world.balls[selected].borrow().position;

                draw_ball(canvas, ball_pos, 0.15, &resources.aimcircle);

                draw_line(canvas, ball_pos, move_vector, 0.01);
            }
            if input::is_key_pressed(Key::Space) {
                game.state = GameState::Running;
                audio::play(&resources.sounds_shot, 1.);

                let moves = mem::replace(&mut game.moves, BTreeMap::new());

                game.world.launch_round(moves);
            }
        }
        GameState::Running => {
            draw_game(canvas, game, resources);
        }
        GameState::GameOver => {
            draw_game(canvas, game, resources);
            canvas.draw_text(
                Vec2::new(0.1, 0.3),
                0.4,
                "Game Over",
                &mut resources.font,
                color::WHITE,
                &canvas.white_texture(),
            );

            if input::is_key_pressed(Key::Space) {
                game.state = GameState::Shopping
            }
        }
        GameState::Shopping => {
            canvas.camera_view(table_size / 2., table_size.x / 2.);

            canvas.draw_rect(
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, 0.5),
                color::rgb(1., 1., 1.),
                &canvas.white_texture(),
            );
        }
        GameState::Menu => {
            canvas.draw_text(
                Vec2::new(0.1, 0.3),
                0.4,
                "SPOOL",
                &mut resources.font,
                color::WHITE,
                &canvas.white_texture(),
            );

            if input::is_key_pressed(Key::Space) {
                game.state = GameState::Playing
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

    let mut resources = Resources::load(&mut canvas).await;

    let mut game = Game {
        moves: BTreeMap::new(),
        world: World::new(0, 0, 0, 0),
        state: GameState::Menu,
        selected: None,
        aim_assist_level: 0,
        max_speed_level: 4,
        profitability_level: 0,
        start_mass_level: 0,
        sliding_level: 0,
        total_money: 0,
    };
    let mut tick_scheduler: TickScheduler = TickScheduler::new(Duration::from_millis(1));
    draw_scheduler::set_on_draw(move || {
        for _ in 0..tick_scheduler.tick_count() {
            game_tick(&mut game, &mut resources);
        }

        render_tick(&mut canvas, &mut game, &mut resources);
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
