extern crate rustc_serialize;
extern crate hyper;
#[cfg(test)]
extern crate mockito;

pub mod client;
pub mod board;
pub mod list;
pub mod card;
pub mod label;

pub use client::Client;
pub use board::Board;
pub use list::List;
pub use card::Card;
pub use label::Label;

#[derive(PartialEq, Debug)]
pub enum Error {
    NotFound,
    Unauthorized,
    TooManyRequests,
    InvalidRequest(String),
    Json(String),
    Unknown(String),
}
