use crate::dom::window;
use glam::IVec2;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
    rc::Rc,
};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{AddEventListenerOptions, Event, KeyboardEvent, MouseEvent, WheelEvent};

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Debug)]
pub enum Key {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    MetaLeft,
    MetaRight,
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    CapsLock,
}

impl Key {
    /// Create a Key from the given code. Returns `None` if the code is unknown.
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        Some(match code {
            "Digit0" => Self::Digit0,
            "Digit1" => Self::Digit1,
            "Digit2" => Self::Digit2,
            "Digit3" => Self::Digit3,
            "Digit4" => Self::Digit4,
            "Digit5" => Self::Digit5,
            "Digit6" => Self::Digit6,
            "Digit7" => Self::Digit7,
            "Digit8" => Self::Digit8,
            "Digit9" => Self::Digit9,
            "KeyA" => Self::A,
            "KeyB" => Self::B,
            "KeyC" => Self::C,
            "KeyD" => Self::D,
            "KeyE" => Self::E,
            "KeyF" => Self::F,
            "KeyG" => Self::G,
            "KeyH" => Self::H,
            "KeyI" => Self::I,
            "KeyJ" => Self::J,
            "KeyK" => Self::K,
            "KeyL" => Self::L,
            "KeyM" => Self::M,
            "KeyN" => Self::N,
            "KeyO" => Self::O,
            "KeyP" => Self::P,
            "KeyQ" => Self::Q,
            "KeyR" => Self::R,
            "KeyS" => Self::S,
            "KeyT" => Self::T,
            "KeyU" => Self::U,
            "KeyV" => Self::V,
            "KeyW" => Self::W,
            "KeyX" => Self::X,
            "KeyY" => Self::Y,
            "KeyZ" => Self::Z,
            "ShiftLeft" => Self::ShiftLeft,
            "ShiftRight" => Self::ShiftRight,
            "ControlLeft" => Self::ControlLeft,
            "ControlRight" => Self::ControlRight,
            "AltLeft" => Self::AltLeft,
            "AltRight" => Self::AltRight,
            "MetaLeft" => Self::MetaLeft,
            "MetaRight" => Self::MetaRight,
            "Enter" => Self::Enter,
            "Escape" => Self::Escape,
            "Backspace" => Self::Backspace,
            "Tab" => Self::Tab,
            "Space" => Self::Space,
            "ArrowUp" => Self::ArrowUp,
            "ArrowDown" => Self::ArrowDown,
            "ArrowLeft" => Self::ArrowLeft,
            "ArrowRight" => Self::ArrowRight,
            "CapsLock" => Self::CapsLock,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Debug)]
pub enum Button {
    Left,
    Middle,
    Right,
    Forward,
    Backward,
}

impl Button {
    /// Create a Button from the given code. Returns `None` if the code is unknown.
    #[must_use]
    pub const fn from_code(code: i16) -> Option<Self> {
        Some(match code {
            0 => Self::Left,
            1 => Self::Middle,
            2 => Self::Right,
            3 => Self::Forward,
            4 => Self::Backward,
            _ => return None,
        })
    }
}

struct Input {
    keys_down: Rc<RefCell<BTreeSet<Key>>>,
    keys_pressed: Rc<RefCell<BTreeSet<Key>>>,
    buttons_down: Rc<RefCell<BTreeSet<Button>>>,
    buttons_pressed: Rc<RefCell<BTreeSet<Button>>>,
    wheel_move: Rc<Cell<f64>>,
    position: Rc<Cell<IVec2>>,
}

impl Input {
    #[must_use]
    fn new() -> Self {
        let window = window();

        let keys_down = Rc::new(RefCell::new(BTreeSet::new()));
        let keys_pressed = Rc::new(RefCell::new(BTreeSet::new()));
        let buttons_down = Rc::new(RefCell::new(BTreeSet::new()));
        let buttons_pressed = Rc::new(RefCell::new(BTreeSet::new()));
        let wheel_move = Rc::new(Cell::new(0.0));
        let position = Rc::new(Cell::new(IVec2::ZERO));

        window
            .add_event_listener_with_callback_and_bool(
                "contextmenu",
                Closure::wrap(Box::new(move |event: MouseEvent| {
                    event.prevent_default();
                }) as Box<dyn Fn(MouseEvent)>)
                .into_js_value()
                .unchecked_ref(),
                true,
            )
            .unwrap();

        let buttons_down_clone = buttons_down.clone();
        let buttons_pressed_clone = buttons_pressed.clone();
        window
            .add_event_listener_with_callback_and_bool(
                "mousedown",
                Closure::wrap(Box::new(move |event: MouseEvent| {
                    event.prevent_default();
                    if let Some(button) = Button::from_code(event.button()) {
                        buttons_down_clone.borrow_mut().insert(button);
                        buttons_pressed_clone.borrow_mut().insert(button);
                    }
                }) as Box<dyn Fn(MouseEvent)>)
                .into_js_value()
                .unchecked_ref(),
                true,
            )
            .unwrap();

        let buttons_down_clone = buttons_down.clone();
        window
            .add_event_listener_with_callback(
                "mouseup",
                Closure::wrap(Box::new(move |event: MouseEvent| {
                    if let Some(button) = Button::from_code(event.button()) {
                        buttons_down_clone.borrow_mut().remove(&button);
                    }
                }) as Box<dyn Fn(MouseEvent)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        let position_clone = position.clone();
        window
            .add_event_listener_with_callback(
                "mousemove",
                Closure::wrap(Box::new(move |event: MouseEvent| {
                    position_clone.set(IVec2::new(event.page_x(), event.page_y()));
                }) as Box<dyn Fn(MouseEvent)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        let wheel_move_clone = wheel_move.clone();

        let wheel_event_listener_options = AddEventListenerOptions::new();
        wheel_event_listener_options.set_passive(false);
        window
            .add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                Closure::wrap(Box::new(move |event: WheelEvent| {
                    wheel_move_clone.set(event.delta_y());
                    event.prevent_default();
                }) as Box<dyn Fn(WheelEvent)>)
                .into_js_value()
                .unchecked_ref(),
                &wheel_event_listener_options,
            )
            .unwrap();

        let keys_down_clone = keys_down.clone();
        let keys_pressed_clone = keys_pressed.clone();
        window
            .add_event_listener_with_callback(
                "keydown",
                Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    event.prevent_default();
                    if let Some(key) = Key::from_code(event.code().as_str()) {
                        keys_down_clone.borrow_mut().insert(key);
                        keys_pressed_clone.borrow_mut().insert(key);
                    }
                }) as Box<dyn Fn(KeyboardEvent)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        let keys_down_clone = keys_down.clone();
        window
            .add_event_listener_with_callback(
                "keyup",
                Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    if let Some(key) = Key::from_code(event.code().as_str()) {
                        keys_down_clone.borrow_mut().remove(&key);
                    }
                }) as Box<dyn Fn(KeyboardEvent)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        let keys_down_clone = keys_down.clone();
        let buttons_down_clone = buttons_down.clone();
        window
            .add_event_listener_with_callback(
                "blur",
                Closure::wrap(Box::new(move |_: Event| {
                    keys_down_clone.borrow_mut().clear();
                    buttons_down_clone.borrow_mut().clear();
                }) as Box<dyn Fn(Event)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        Self {
            keys_down,
            keys_pressed,
            buttons_down,
            buttons_pressed,
            wheel_move,
            position,
        }
    }

    #[must_use]
    pub fn is_button_down(&self, button: Button) -> bool {
        self.buttons_down.borrow().contains(&button)
    }

    #[must_use]
    pub fn is_button_pressed(&self, button: Button) -> bool {
        self.buttons_pressed.borrow_mut().remove(&button)
    }

    #[must_use]
    pub fn wheel_scroll(&self) -> f64 {
        self.wheel_move.replace(0.0)
    }

    #[must_use]
    pub fn position(&self) -> IVec2 {
        self.position.get()
    }

    #[must_use]
    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_down.borrow().contains(&key)
    }

    #[must_use]
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.borrow_mut().remove(&key)
    }

    #[must_use]
    pub fn reset_pressed(&self) {
        self.keys_pressed.borrow_mut().clear();
        self.buttons_pressed.borrow_mut().clear();
    }
}

thread_local! {
    pub static INPUT: Input = Input::new();
}

#[must_use]
pub fn is_key_down(key: Key) -> bool {
    INPUT.with(|input| input.is_key_down(key))
}

#[must_use]
pub fn is_key_pressed(key: Key) -> bool {
    INPUT.with(|input| input.is_key_pressed(key))
}

#[must_use]
pub fn is_button_down(button: Button) -> bool {
    INPUT.with(|input| input.is_button_down(button))
}

#[must_use]
pub fn is_button_pressed(button: Button) -> bool {
    INPUT.with(|input| input.is_button_pressed(button))
}

pub fn wheel_scroll() -> f64 {
    INPUT.with(Input::wheel_scroll)
}

pub fn mouse_position() -> IVec2 {
    INPUT.with(Input::position)
}

pub fn reset_pressed() {
    INPUT.with(Input::reset_pressed);
}
