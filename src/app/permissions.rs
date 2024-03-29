use std::fmt::Display;
use strum::{Display, EnumString};

pub trait Permission where Self: Display {}

#[derive(EnumString , Display)]
#[strum(prefix="Product|")]
pub enum ProductP {
    Store,
    Update,
    Delete,
}

impl Permission for ProductP {}


