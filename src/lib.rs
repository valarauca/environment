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
//! * `0x[a-fA-F0-9]`: Will be converted to `i64`, and be assumed to be hexidecimal.
//! * `0o[0-7]+`: Will be converted to `i64`, and be assumed to be octal.
//! * `[+|-]?[0-9]`: Will be converted to `i64`, and assumed to be decimal.
//! * Most common patterns of floating point (`0.5`, `.5`, `3.14`, `-3.14`, `2.5E10`, etc.) will be converted to `f64`
//! * IP addresses will be converted to `IpAddr`.
//! * SocketAddresses (i.e.: `127.0.0.1:666`) will be converted into `SocketAddr`.
//! * `:` will be treated as an array delimator (as convention suggests)
//! * `true`, `false`, `T`, `F`, `TRUE`, `FALSE` will become `bool`.
//!
//! Orginal values are faithfully perserved so if the orginal string is wanted it will always be recoverable.

#![allow(clippy::needless_lifetimes, clippy::match_ref_pats)]

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
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
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
impl Value {
    /// as_str will always succeed as it will always fallback to the `String` stored with each value.
    pub fn as_str<'a>(&'a self) -> Option<&'a str> {
        match self {
            &Value::String(ref s)
            | &Value::Bool(_, ref s)
            | &Value::Int(_, ref s)
            | &Value::Float(_, ref s)
            | &Value::SocketAddr(_, ref s)
            | &Value::IpAddr(_, ref s)
            | &Value::Array(_, ref s) => Some(s),
        }
    }

    pub fn as_bool<'a>(&'a self) -> Option<&'a bool> {
        match self {
            &Value::Bool(ref b, _) => Some(b),
            _ => None,
        }
    }
    pub fn as_int<'a>(&'a self) -> Option<&'a i64> {
        match self {
            &Value::Int(ref i, _) => Some(i),
            _ => None,
        }
    }
    pub fn as_float<'a>(&'a self) -> Option<&'a f64> {
        match self {
            &Value::Float(ref f, _) => Some(f),
            _ => None,
        }
    }
    pub fn as_socket<'a>(&'a self) -> Option<&'a SocketAddr> {
        match self {
            &Value::SocketAddr(ref s, _) => Some(s),
            _ => None,
        }
    }
    pub fn as_ipv4<'a>(&'a self) -> Option<&'a Ipv4Addr> {
        match self {
            &Value::IpAddr(IpAddr::V4(ref i), _) => Some(i),
            _ => None,
        }
    }
    pub fn as_ipv6<'a>(&'a self) -> Option<&'a Ipv6Addr> {
        match self {
            &Value::IpAddr(IpAddr::V6(ref i), _) => Some(i),
            _ => None,
        }
    }
}

/// Envir is a representation of the environment. But a handful of
/// conventions and opinions are applied to the data which is found
/// within the environment.
///
/// This type will not refresh, or re-sync with the environment following
/// start up. This is because modifying your environment key-values post
/// startup is generally considered an anti-pattern.
///
/// Envir is sync/send safe as internally it uses `Arc<T>`. None of its
/// methods allow for mutation of the interior data.
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
    pub fn get_bool<'a, S: AsRef<str>>(&'a self, k: &S) -> Option<&'a bool> {
        self.get(k)
            .into_iter()
            .flat_map(|value| value.as_bool())
            .next()
    }
    pub fn get_int<'a, S: AsRef<str>>(&'a self, k: &S) -> Option<&'a i64> {
        self.get(k)
            .into_iter()
            .flat_map(|value| value.as_int())
            .next()
    }
    pub fn get_float<'a, S: AsRef<str>>(&'a self, k: &S) -> Option<&'a f64> {
        self.get(k)
            .into_iter()
            .flat_map(|value| value.as_float())
            .next()
    }
    pub fn get_ipv4<'a, S: AsRef<str>>(&'a self, k: &S) -> Option<&'a Ipv4Addr> {
        self.get(k)
            .into_iter()
            .flat_map(|value| value.as_ipv4())
            .next()
    }
    pub fn get_ipv6<'a, S: AsRef<str>>(&'a self, k: &S) -> Option<&'a Ipv6Addr> {
        self.get(k)
            .into_iter()
            .flat_map(|value| value.as_ipv6())
            .next()
    }
    pub fn get_str<'a, S: AsRef<str>>(&'a self, k: &S) -> Option<&'a str> {
        self.get(k)
            .into_iter()
            .flat_map(|value| value.as_str())
            .next()
    }
}

impl Value {
    // constructs a new value.
    //
    // iterates over the possible value constructors, and lazily constructs the first it encounters
    fn new(arg: String) -> Self {
        Option::None
            .into_iter()
            .chain(parse_int(&arg).map(|val| Self::Int(val, arg.clone())))
            .chain(parse_float(&arg).map(|val| Self::Float(val, arg.clone())))
            .chain(parse_bool(&arg).map(|val| Self::Bool(val, arg.clone())))
            .chain(parse_socket(&arg).map(|val| Self::SocketAddr(val, arg.clone())))
            .chain(parse_ip(&arg).map(|val| Self::IpAddr(val, arg.clone())))
            .chain(Value::split(&arg))
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
