pub enum Event {
    Quit,
    KeyDown(Keycode),
    KeyUp(Keycode),
}

pub enum Keycode {
    Up,
    Down,
    Left,
    Right,
    Escape,
}
