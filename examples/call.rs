//! Example on how having a `Callable` trait should help the chained calls and
//! return values/type.
//!
//! Thanks to kangalioo#9108 on the Rust Discord for the base example, now heavily modified.
//!

// std library
use std::fmt::Debug;

// External crates
use anyhow::Result;
use serde::de;
use serde_json::from_str;

/// Generic return type
///
#[derive(Clone, Copy, Debug)]
struct Callable<'c, T> {
    tag: &'c str,
    res: T,
}

/// Implementation.
///
impl<'c, T> Callable<'c, T> {
    /// This is the generic return type call
    ///
    pub fn call(self) -> T
    where
        T: Debug,
    {
        println!("res={:?}", self.res);
        self.res
    }
}

/// This will be our RequestBuilder thingy
///
#[derive(Debug, Copy, Clone)]
struct Request<'r> {
    t: u32,
    opts: &'r str,
}

/// And how we implement both `get()` and `list()`
///
impl<'r> Request<'r> {
    fn new() -> Self {
        Self { t: 0, opts: "" }
    }

    pub fn with(mut self, opts: &'r str) -> Self {
        self.opts = opts;
        self
    }

    fn get<T>(self, n: u32) -> Callable<'r, T>
    where
        T: Copy,
    {
        let r = getint(n);
        Callable { tag: "get", res: r }
    }

    fn list<T>(self) -> Callable<'r, Vec<T>>
    where
        T: Copy,
    {
        let mut r: Vec<T> = getresults();

        Callable {
            tag: "list",
            res: r,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Probe<'p> {
    s: &'p str,
}

#[derive(Copy, Clone, Debug)]
struct Key {
    n: u32,
}

#[derive(Copy, Clone, Debug)]
struct Client {
    id: u32,
}

impl<'c> Client {
    fn new(id: u32) -> Self {
        Client { id: id }
    }

    fn probe(self) -> Request<'c> {
        Request {
            opts: "probe",
            t: 1,
        }
    }

    fn keys(self) -> Request<'c> {
        Request { opts: "keys", t: 2 }
    }
}

fn getresults<T>() -> Vec<T>
where
    T: Copy + for<'de> serde::Deserialize<'de>,
{
    let res = "[Probe{}, Probe{}]".to_string();
    serde_json::from_str(&res).unwrap()
}

fn getint<T>(p: u32) -> T {
    42 + p
}

/// Practical example.
///
fn main() -> Result<()> {
    let c = Client::new(1);

    let r: Probe = c.probe().get(5).call();
    let l: Vec<Key> = c.keys().with("aa").list().call();

    println!("r={:?}", r);
    println!("r={:?}", l);

    Ok(())
}
