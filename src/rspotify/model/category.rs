//! All object related to category
use super::{Image, Page};
use serde::{Deserialize, Serialize};

/// Category object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}

/// Intermediate categories wrapped by page object
#[derive(Deserialize)]
pub struct PageCategory {
    pub categories: Page<Category>,
}
