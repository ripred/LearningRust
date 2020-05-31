//! This module contains french phrases
//!
//! # Examples
//! ```
//! extern crate phrases;
//! use phrases::greetings::french;
//!
//! let user_name = "John";
//!
//! println!("French: {}, {}",
//!   french::hello(),
//!   user_name);
//! 
//! println!("French: {}, {}",
//!   french::goodbye(),
//!   user_name);
//! 

/// our `hello()` function
/// # Examples
/// ```
/// extern crate phrases;
/// use phrases::greetings::french;
///
/// println!("{}", french::hello());
/// ```
pub fn hello() -> String { "bonjour".to_string() }


/// our `goodbye()` function
/// # Examples
/// ```
/// extern crate phrases;
/// use phrases::greetings::french;
///
/// println!("{}", french::goodbye());
/// ```
pub fn goodbye() -> String { "au revoir".to_string() }
