//! Environment is an extremely simple crate which handles some
//! "common sense" type safety to the ever present environment
//! vars values.
//! This crate is based off of convention, and may not fit all
//! usecases, but it should fit most.
//! This create was created as I've written the code for it
//! on at least 3 seperate occasions (not counting this crate)
//! Therefore it seemed wise to create a static stand-alone
//! abstraction to manage this.
//!
//! This crate will preform some basic serialization of Envir
//! Var key-values in the following manner:
//!
//! * `0x[a-fA-F0-9]`: Will be converted to `i64`.
//! * `0o[0-7]+`: Will be converted to `i64`.
//! * `[+|-]?[0-9]`: Will be converted to `i64`.
//! * Most common patterns of floating point (`0.5`, `.5`, `3.14`, `-3.14`, `2.5E10`, etc.) will be converted to `f64`
//! * IP addresses will be converted to `IpAddr`.
//! * SocketAddresses (i.e.: `127.0.0.1:666`) will be converted into `SocketAddr`.
//! * `:` will be treated as an array delimator (as convention suggests)
//! * `true`, `false`, `T`, `F`, `TRUE`, `FALSE` will become `bool`.
//!
//! Orginal values are faithfully perserved so if the orginal string is wanted it will always be recoverable.

extern crate regex;
#[macro_use]
extern crate lazy_static;

mod bool;
use self::bool::parse_bool;
mod floats;
use self::floats::parse_float;
mod int;
use self::int::parse_int;
mod socketaddr;
use self::socketaddr::{parse_ip, parse_socket};

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

/// Value contains the pre & post encoding information about a value.
///
/// Some information about the variables maybe dropped when they're converted into
/// a type safe format, therefore the orginal value is perserved if that is preferable.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool, String),
    Int(i64, String),
    Float(f64, String),
    SocketAddr(SocketAddr, String),
    IpAddr(IpAddr, String),
    Array(Box<[Value]>, String),
}

/// Envir is a representation of the environment. But a handful of
/// conventions and opinions are applied to the data which is found
/// within the environment.
///
/// This type will not refresh, or re-sync with the environment following
/// start up. This is because modifying your environment key-values post
/// startup is generally considered an anti-pattern.
///
/// Envir is sync/send safe as internally it uses `Arc<T>`.
#[derive(Clone)]
pub struct Envir {
    data: Arc<HashMap<String, Value>>,
}
impl Default for Envir {
    fn default() -> Envir {
        Envir {
            data: Arc::new(
                ::std::env::vars()
                    .map(|(key, value)| (key, Value::new(value)))
                    .collect::<HashMap<String, Value, _>>(),
            ),
        }
    }
}
unsafe impl Send for Envir {}
unsafe impl Sync for Envir {}
impl Envir {
    /// get will return a pointer to a value if one is found.
    pub fn get<'a, S: AsRef<str>>(&'a self, k: &S) -> Option<&'a Value> {
        self.data.as_ref().get(k.as_ref())
    }
}

impl Value {
    // constructs a new value.
    //
    // iterates over the possible value constructors, and lazily constructs the first it encounters
    fn new(arg: String) -> Self {
        Option::None
            .into_iter()
            .chain(Value::split(&arg))
            .chain(parse_int(&arg).map(|val| Self::Int(val, arg.clone())))
            .chain(parse_float(&arg).map(|val| Self::Float(val, arg.clone())))
            .chain(parse_bool(&arg).map(|val| Self::Bool(val, arg.clone())))
            .chain(parse_socket(&arg).map(|val| Self::SocketAddr(val, arg.clone())))
            .chain(parse_ip(&arg).map(|val| Self::IpAddr(val, arg.clone())))
            .next()
            .unwrap_or(Self::String(arg))
    }

    // split handles the operation of splitting a value by `:` a common convention
    fn split<S: AsRef<str>>(arg: &S) -> Option<Self> {
        if !arg.as_ref().contains(':') {
            return None;
        }
        let collection = arg
            .as_ref()
            .split(':')
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| Value::new(item.to_string()))
            .collect::<Vec<Value>>();
        match collection.len() {
            0 => None,
            1 => Some(collection[0].clone()),
            _ => Some(Value::Array(
                collection.into_boxed_slice(),
                arg.as_ref().to_string(),
            )),
        }
    }
}
