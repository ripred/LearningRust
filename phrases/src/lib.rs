//! This crate contains english and french phrases
//!
//! # Examples
//! ```
//! extern crate phrases;
//! use phrases::greetings::english;
//! use phrases::greetings::french;
//!
//! let user_name = "John";
//!
//! println!("English: {}, {}",
//!   english::hello(),
//!   user_name);
//! 
//! println!("English: {}, {}",
//!   english::goodbye(),
//!   user_name);
//! 
//! println!("French: {}, {}",
//!   french::hello(),
//!   user_name);
//! 
//! println!("French: {}, {}",
//!   french::goodbye(),
//!   user_name);
//! 

pub mod greetings {
    pub mod english;
    pub mod french;
}
