/// Display HAT Mini button names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonId {
    A,
    B,
    X,
    Y,
}

/// Type of button press
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonPress {
    Short,
    Long,
    Double,
}

/// The button pressed and the type of press
#[derive(Debug, Clone, Copy)]
pub struct InputEvent {
    pub id: ButtonId,
    pub press_type: ButtonPress,
}
