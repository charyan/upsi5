use marmalade::{
    audio::{self, Audio},
    font::{self, Font},
    image,
    render::canvas2d::{Canvas2d, TextureRect},
};

pub struct Resources {
    pub pool_table: TextureRect,
    pub slimeball: TextureRect,
    pub balls: [TextureRect; 6],
    pub aimcircle: TextureRect,
    pub coin: TextureRect,
    pub coin_flat: TextureRect,
    pub speed_upgrade: TextureRect,
    pub slide_upgrade: TextureRect,
    pub heavy_upgrade: TextureRect,
    pub coin_upgrade: TextureRect,
    pub aim_upgrade: TextureRect,
    pub button: TextureRect,
    pub title_screen: TextureRect,
    pub tuto_1: TextureRect,
    pub tuto_2: TextureRect,
    pub hand_open: TextureRect,
    pub hand_closed: TextureRect,
    pub font: Font,
    pub sounds_slimeslime: Audio,
    pub sounds_coin: Audio,
    pub sounds_shot: Audio,
    pub sounds_border: Audio,
    pub sounds_enemyenemy: Audio,
    pub sounds_slimeenemy: Audio,
    pub sounds_button: Audio,
    pub main_theme: Audio,
    pub sounds_falling: Audio,
}

async fn load_texture(canvas: &mut Canvas2d, bytes: &[u8]) -> TextureRect {
    canvas.create_texture(&image::from_bytes(bytes).await)
}

impl Resources {
    pub async fn load(canvas: &mut Canvas2d) -> Self {
        let pool_table = load_texture(canvas, include_bytes!("../assets/pool_table.png")).await;
        let slimeball = load_texture(canvas, include_bytes!("../assets/slimeball.png")).await;

        let endball = load_texture(canvas, include_bytes!("../assets/endball.png")).await;
        let ball1 = load_texture(canvas, include_bytes!("../assets/ball1.png")).await;
        let ball2 = load_texture(canvas, include_bytes!("../assets/ball2.png")).await;
        let ball3 = load_texture(canvas, include_bytes!("../assets/ball3.png")).await;
        let ball4 = load_texture(canvas, include_bytes!("../assets/ball4.png")).await;
        let ball5 = load_texture(canvas, include_bytes!("../assets/ball5.png")).await;
        let aimcircle = load_texture(canvas, include_bytes!("../assets/aimcircle.png")).await;
        let coin: TextureRect = load_texture(canvas, include_bytes!("../assets/coin.png")).await;
        let coin_flat = load_texture(canvas, include_bytes!("../assets/coinflat.png")).await;
        let speed_upgrade =
            load_texture(canvas, include_bytes!("../assets/speedupgrade.png")).await;
        let slide_upgrade =
            load_texture(canvas, include_bytes!("../assets/slideupgrade.png")).await;
        let heavy_upgrade =
            load_texture(canvas, include_bytes!("../assets/heavyupgrade.png")).await;
        let coin_upgrade = load_texture(canvas, include_bytes!("../assets/coinupgrade.png")).await;
        let aim_upgrade = load_texture(canvas, include_bytes!("../assets/aimupgrade.png")).await;
        let button = load_texture(canvas, include_bytes!("../assets/button.png")).await;
        let title_screen = load_texture(canvas, include_bytes!("../assets/titlescreen.png")).await;
        let tuto_1 = load_texture(canvas, include_bytes!("../assets/tutomecanics1.png")).await;
        let tuto_2 = load_texture(canvas, include_bytes!("../assets/tutosendgame2.png")).await;

        let hand_open = load_texture(canvas, include_bytes!("../assets/handopen.png")).await;
        let hand_closed = load_texture(canvas, include_bytes!("../assets/handclosed.png")).await;

        let font = font::from_bytes(include_bytes!("../assets/modak.ttf"));

        let sounds_slimeslime =
            audio::from_bytes(include_bytes!("../sounds/slimeslime.flac")).await;
        let sounds_coin = audio::from_bytes(include_bytes!("../sounds/takecoin.flac")).await;
        let sounds_shot = audio::from_bytes(include_bytes!("../sounds/shoot.flac")).await;
        let sounds_border = audio::from_bytes(include_bytes!("../sounds/border.flac")).await;
        let sounds_enemyenemy =
            audio::from_bytes(include_bytes!("../sounds/enemyenemy.flac")).await;
        let sounds_slimeenemy =
            audio::from_bytes(include_bytes!("../sounds/slimeenemy.flac")).await;
        let sounds_button = audio::from_bytes(include_bytes!("../sounds/button.flac")).await;
        let sounds_falling = audio::from_bytes(include_bytes!("../sounds/falling.flac")).await;

        let main_theme = audio::from_bytes(include_bytes!("../sounds/ludwig.flac")).await;

        Self {
            pool_table,
            slimeball,
            balls: [endball, ball1, ball2, ball3, ball4, ball5],
            aimcircle,
            coin,
            coin_flat,
            font,
            sounds_coin,
            sounds_shot,
            sounds_slimeslime,
            aim_upgrade,
            coin_upgrade,
            heavy_upgrade,
            slide_upgrade,
            speed_upgrade,
            button,
            hand_open,
            hand_closed,
            sounds_border,
            sounds_enemyenemy,
            sounds_slimeenemy,
            sounds_button,
            main_theme,
            sounds_falling,
            title_screen,
            tuto_1,
            tuto_2,
        }
    }
}
