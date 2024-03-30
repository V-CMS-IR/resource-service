#![allow(clippy::upper_case_acronyms)]

use std::fmt::Display;
use strum::{Display, EnumString};

pub trait Permission where Self: Display {}

#[derive(EnumString , Display)]
#[strum(prefix="PRODUCT_")]
pub enum ProductP {
    STORE,
    UPDATE,
    DELETE,
}

impl Permission for ProductP {}


