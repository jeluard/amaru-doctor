use strum::{Display, EnumIter};

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum NavMode {
    Browse,
    Search,
}
