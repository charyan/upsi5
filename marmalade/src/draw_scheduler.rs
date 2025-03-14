use crate::dom::window;
use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

struct DrawScheduler {
    draw_closure: Rc<RefCell<Box<dyn FnMut()>>>,
}

impl DrawScheduler {
    #[must_use]
    fn new() -> Self {
        let draw_closure: Rc<RefCell<Box<dyn FnMut()>>> = Rc::new(RefCell::new(Box::new(|| {})));

        let request_animation_frame_closure = Rc::new(OnceCell::<JsValue>::new());

        let request_animation_frame_closure_clone = request_animation_frame_closure.clone();
        let draw_closure_clone = draw_closure.clone();

        request_animation_frame_closure
            .set(
                Closure::<dyn FnMut()>::new(move || {
                    (draw_closure_clone.borrow_mut())();

                    window()
                        .request_animation_frame(
                            request_animation_frame_closure_clone
                                .get()
                                .unwrap()
                                .unchecked_ref(),
                        )
                        .unwrap();
                })
                .into_js_value(),
            )
            .unwrap();

        window()
            .request_animation_frame(
                request_animation_frame_closure
                    .get()
                    .unwrap()
                    .unchecked_ref(),
            )
            .unwrap();

        Self { draw_closure }
    }
}

thread_local! {
    static DRAW_SCHEDULER: DrawScheduler = DrawScheduler::new();
}

pub fn set_on_draw<T: FnMut() + 'static>(closure: T) {
    DRAW_SCHEDULER.with(|d| *d.draw_closure.borrow_mut() = Box::new(closure));
}

pub fn clear_on_draw() {
    set_on_draw(|| {});
}
