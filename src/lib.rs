// This file is only used for running tests
// It's a nice little workaround because it lets us
// include standard library and stuff.
pub mod mem;
pub mod math;
pub mod http_models;
pub mod system;

#[cfg(test)]
pub fn err() {}