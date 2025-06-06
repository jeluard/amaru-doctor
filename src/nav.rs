use strum::Display;

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum NavMode {
    Browse,
    Search,
}
