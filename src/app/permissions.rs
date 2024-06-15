#![allow(clippy::upper_case_acronyms)]

use std::fmt::Display;
use strum::{Display, EnumString};

pub trait Permission where Self: Display {}

#[derive(EnumString , Display)]
#[strum(prefix="PRODUCT_")]
pub enum ProductPermissions {
    STORE,
    UPDATE,
    DELETE,
}

#[derive(EnumString , Display)]
#[strum(prefix="CATEGORY_")]
pub enum CategoryPermissions{
    STORE,
    UPDATE,
    DELETE
}

// TODO remove these shity trait and found another way
impl Permission for ProductPermissions {}
impl Permission for CategoryPermissions{}

