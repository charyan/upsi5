use marmalade::{
    audio,
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
    pub font: Font,
    pub sounds_slimeslime: audio::Audio,
    pub sounds_coin: audio::Audio,
    pub sounds_shot: audio::Audio,
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
        let coin = load_texture(canvas, include_bytes!("../assets/coin.png")).await;

        let font = font::from_bytes(font::MONOGRAM);

        let sounds_slimeslime =
            audio::from_bytes(include_bytes!("../sounds/slimeslime.flac")).await;
        let sounds_coin = audio::from_bytes(include_bytes!("../sounds/takecoin.flac")).await;
        let sounds_shot = audio::from_bytes(include_bytes!("../sounds/shoot.flac")).await;

        Self {
            pool_table,
            slimeball,
            balls: [endball, ball1, ball2, ball3, ball4, ball5],
            aimcircle,
            coin,
            font,
            sounds_coin,
            sounds_shot,
            sounds_slimeslime,
        }
    }
}
