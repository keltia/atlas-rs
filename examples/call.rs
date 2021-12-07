//! Example on how having a `Callable` trait should help the chained calls and
//! return values/type.
//!
//! Thanks to kangalioo#9108 on the Rust Discord for the base example, now heavily modified.
//!

use std::fmt::Debug;
use anyhow::Result;

/// Generic return type
///
#[derive(Clone, Copy, Debug)]
struct Callable<'c, T> {
    tag: &'c str,
    res: T }

/// Implementation.
///
impl<'c, T> Callable<'c, T> {
    /// This is the generic return type call
    ///
    pub fn call(self) -> T
        where T: Debug,
    {
        println!("res={:?}", self.res);
        self.res
    }

}

/// This will be our RequestBuilder thingy
///
#[derive(Debug, Copy, Clone)]
struct Request<'r, T> {
    t: u32,
    opts: &'r str,
    val: T,
}

/// And how we implement both `get()` and `list()`
///
impl<'r, T> Request<'r, T> {
    fn new(v: T) -> Self {
        Self { t: 0, opts: "", val: v }
    }

    pub fn with(mut self, opts: &'r str) -> Self
        where T: Debug,
    {
        self.opts = opts.as_str();
        self
    }

    fn get(self, n: u32) -> Callable<'r, T>
        where T: Copy,
    {
        Callable { tag: "get", res: self.val }
    }

    fn list(self) -> Callable<'r, Vec<T>>
        where T: Copy,
    {
        Callable { tag: "list", res: vec![self.val, self.val ] }
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
        Client {
            id: id,
        }
    }

    fn probe<T>(self) -> Request<'c, T> {
        Request {
            opts: "",
            t: 1,
            val: self.id.into(),
        }
    }

    fn keys<T>(self) -> Request<'c, T> {
        Request {
            opts: "",
            t: 2,
            val: self.id.into(),
        }
    }
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
