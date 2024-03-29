#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(async_closure)]
#![feature(async_fn_in_trait)]
#![feature(exclusive_range_pattern)]
#![feature(adt_const_params)]
#![feature(inherent_associated_types)]

extern crate self as howitt;

pub mod ext;
pub mod models;
pub mod repos;
pub mod services;
