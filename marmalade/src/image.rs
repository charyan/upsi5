use crate::dom::window;
use js_sys::{Array, Uint8Array};
use wasm_bindgen::{JsCast, __rt::IntoJsResult};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, HtmlImageElement, ImageBitmap};

/// Load an image from the network at the given src
///
/// # Errors
///
/// Returns Err if the image couldn't be loaded from src
pub async fn load(src: &str) -> Result<ImageBitmap, ()> {
    let img = HtmlImageElement::new().unwrap();
    img.set_src(src);

    if JsFuture::from(img.decode()).await.is_ok() {
        if let Ok(value) = JsFuture::from(
            window()
                .create_image_bitmap_with_html_image_element(&img)
                .unwrap(),
        )
        .await
        {
            return Ok(value.dyn_into::<ImageBitmap>().unwrap());
        }
    }

    Err(())
}

pub async fn from_bytes(bytes: &[u8]) -> ImageBitmap {
    let array = Array::new();
    array.push(&Uint8Array::from(bytes).into_js_result().unwrap());

    JsFuture::from(
        window()
            .create_image_bitmap_with_blob(&Blob::new_with_u8_array_sequence(&array).unwrap())
            .unwrap(),
    )
    .await
    .unwrap()
    .dyn_into::<ImageBitmap>()
    .unwrap()
}
