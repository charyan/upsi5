use marmalade::{
    image,
    render::canvas2d::{Canvas2d, TextureRect},
};

pub struct Resources {
    pub pool_table: TextureRect,
    pub slimeball: TextureRect,
    pub ball1: TextureRect,
}

async fn load_texture(canvas: &mut Canvas2d, bytes: &[u8]) -> TextureRect {
    canvas.create_texture(&image::from_bytes(bytes).await)
}

impl Resources {
    pub async fn load(canvas: &mut Canvas2d) -> Self {
        let pool_table = load_texture(canvas, include_bytes!("../assets/pool_table.png")).await;
        let slimeball = load_texture(canvas, include_bytes!("../assets/slimeball.png")).await;
        let ball1 = load_texture(canvas, include_bytes!("../assets/ball1.png")).await;

        Self {
            pool_table,
            slimeball,
            ball1,
        }
    }
}
