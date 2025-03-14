use glam::Vec2;
use marmalade::audio;
use marmalade::dom::window;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::font;
use marmalade::input;
use marmalade::input::Key;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::color;
use marmalade::tick_scheduler::TickScheduler;
use std::cell::RefCell;
use std::time::Duration;

const GRAVITY: Vec2 = Vec2::new(0., 0.0015);
const COLLISION_SMOOTHNESS: f32 = 0.003;

const FRICTION: f32 = 0.999;

struct Ball {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new(position: Vec2, speed: Vec2, radius: f32) -> Self {
        Self {
            position,
            speed,
            radius,
        }
    }

    pub fn tick(&mut self) -> f32 {
        self.speed -= GRAVITY;
        self.position += self.speed;

        self.speed *= FRICTION;

        let window = window();

        let width = window.inner_width().unwrap().as_f64().unwrap() as f32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as f32;

        let mut collision_strength = 0.;

        if self.position.x - self.radius < 0. {
            self.position.x = self.radius;
            self.speed.x = -self.speed.x;
            collision_strength = self.speed.x.abs()
        } else if self.position.x + self.radius > width {
            self.position.x = width - self.radius;
            self.speed.x = -self.speed.x;
            collision_strength = self.speed.x.abs()
        }

        if self.position.y - self.radius < 0. {
            self.position.y = self.radius;
            self.speed.y = -self.speed.y;
            collision_strength = self.speed.y.abs()
        } else if self.position.y + self.radius > height {
            self.position.y = height - self.radius;
            self.speed.y = -self.speed.y;
            collision_strength = self.speed.y.abs()
        }

        collision_strength
    }

    pub fn collide(a: &mut Ball, b: &mut Ball) {
        let dist = a.position - b.position;

        let overlap = a.radius + b.radius - dist.length();

        if overlap > 0. {
            let push = dist.normalize_or_zero() * overlap * COLLISION_SMOOTHNESS;
            a.speed += push;
            b.speed -= push;
        }
    }
}

async fn async_main() {
    let sound = audio::from_bytes(include_bytes!("resources/bounce.flac")).await;

    let mut font = font::from_bytes(font::MONOGRAM);

    dom_stack::set_title("Bouncing Balls");

    let mut balls = Vec::new();

    let main_canvas = dom_stack::create_full_screen_canvas();
    dom_stack::stack_node(&main_canvas);

    let mut canvas = Canvas2d::new(&main_canvas);

    let mut tick_scheduler = TickScheduler::new(Duration::from_millis(1));

    let mut write_instructions = true;

    let white_texture = canvas.white_texture();

    draw_scheduler::set_on_draw(move || {
        if input::is_key_pressed(Key::Space) {
            let win = window();

            balls.push(RefCell::new(Ball::new(
                Vec2::new(
                    30.,
                    win.inner_height().unwrap().as_f64().unwrap() as f32 - 30.,
                ),
                Vec2::new(3., 0.),
                25.,
            )));

            write_instructions = false;
        }

        let mut loudest = 0f32;

        for _ in 0..tick_scheduler.tick_count() {
            for ball in &mut balls {
                let collision_strength = ball.borrow_mut().tick();

                loudest = loudest.max(collision_strength);
            }

            for a in 0..balls.len() {
                for b in (a + 1)..balls.len() {
                    Ball::collide(&mut balls[a].borrow_mut(), &mut balls[b].borrow_mut());
                }
            }
        }

        if loudest > 0.1 {
            audio::play(&sound, (loudest - 0.1).clamp(0., 1.));
        }

        canvas.fit_screen();

        canvas.pixel_perfect_view();

        canvas.clear(color::rgb(0., 0., 0.));

        for ball in &balls {
            let ball = ball.borrow_mut();

            canvas.draw_regular(
                ball.position,
                ball.radius,
                32,
                color::rgb(1., 0.5, 0.),
                &white_texture,
            );
        }

        canvas.flush();

        if write_instructions {
            canvas.draw_text(
                Vec2::new(50., 100.),
                64.,
                "Press SPACE to throw a ball",
                &mut font,
                color::rgb(1., 1., 1.),
                &canvas.white_texture(),
            );
        }
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
