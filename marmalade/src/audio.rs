use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};

thread_local! {
    static CONTEXT: AudioContext = AudioContext::new().unwrap();
}

pub type Audio = AudioBuffer;

pub async fn from_bytes(bytes: &[u8]) -> Audio {
    JsFuture::from(CONTEXT.with(|c| {
        c.decode_audio_data(&Uint8Array::from(bytes).buffer())
            .unwrap()
    }))
    .await
    .unwrap()
    .dyn_into::<AudioBuffer>()
    .unwrap()
}

type SoundHandle = AudioBufferSourceNode;

pub fn play(audio: &Audio, volume: f32) -> SoundHandle {
    CONTEXT.with(|c| {
        let source = c.create_buffer_source().unwrap();
        let gain = c.create_gain().unwrap();

        gain.connect_with_audio_node(&c.destination()).unwrap();

        gain.gain().set_value(volume);

        source.set_buffer(Some(audio));

        source.connect_with_audio_node(&gain).unwrap();

        source.start().unwrap();

        source
    })
}

pub fn play_loop(audio: &Audio, volume: f32) -> SoundHandle {
    CONTEXT.with(|c| {
        let source = c.create_buffer_source().unwrap();
        let gain = c.create_gain().unwrap();

        gain.connect_with_audio_node(&c.destination()).unwrap();

        gain.gain().set_value(volume);

        source.set_buffer(Some(audio));
        source.set_loop(true);

        source.connect_with_audio_node(&gain).unwrap();

        source.start().unwrap();

        source
    })
}
