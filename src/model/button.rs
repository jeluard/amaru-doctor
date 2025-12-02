use crate::states::Action;
use tracing::debug;

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

impl InputEvent {
    pub fn to_action(&self) -> Action {
        match self {
            InputEvent {
                id: ButtonId::A, ..
            } => {
                debug!("Translating Button A press to Up Action");
                Action::Up
            }
            InputEvent {
                id: ButtonId::B, ..
            } => {
                debug!("Translating Button B press to Down Action");
                Action::Down
            }
            InputEvent {
                id: ButtonId::X, ..
            } => {
                debug!("Translating Button X press to Forward Action");
                Action::Forward
            }
            InputEvent {
                id: ButtonId::Y, ..
            } => {
                debug!("Translating Button Y press to Back Action");
                Action::Back
            }
        }
    }
}
