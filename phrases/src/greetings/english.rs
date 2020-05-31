//! This module contains english phrases
//!
//! # Examples
//! ```
//! extern crate phrases;
//! use phrases::greetings::english;
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

/// our `hello()` function
/// # Examples
/// ```
/// let user_name = "John";
/// println!("{}",
///   phrases::greetings::english::hello() );
/// ```
pub fn hello() -> String { "hello".to_string() }


/// our `goodbye()` function
/// # Examples
/// ```
/// let user_name = "John";
/// println!("{}",
///   phrases::greetings::english::goodbye() );
/// ```
pub fn goodbye() -> String { "goodbye".to_string() }
