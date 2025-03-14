use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioContext};

thread_local! {
    static CONTEXT: AudioContext = AudioContext::new().unwrap();
}

pub async fn from_bytes(bytes: &[u8]) -> AudioBuffer {
    JsFuture::from(CONTEXT.with(|c| {
        c.decode_audio_data(&Uint8Array::from(bytes).buffer())
            .unwrap()
    }))
    .await
    .unwrap()
    .dyn_into::<AudioBuffer>()
    .unwrap()
}

pub fn play(audio: &AudioBuffer, volume: f32) {
    CONTEXT.with(|c| {
        let source = c.create_buffer_source().unwrap();
        let gain = c.create_gain().unwrap();

        gain.connect_with_audio_node(&c.destination()).unwrap();

        gain.gain().set_value(volume);

        source.set_buffer(Some(audio));

        source.connect_with_audio_node(&gain).unwrap();

        source.start().unwrap();
    });
}
