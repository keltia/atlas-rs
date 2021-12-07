//! Example on how having a `Callable` trait should help the chained calls and
//! return values/type.
//!
//! Thanks to kangalioo#9108 on Discord for it.
//!

use std::fmt::Debug;
use anyhow::Result;

/// Generic return type
///
#[derive(Debug)]
struct Callable<T> { res: T }

/// Implementation.
///
impl<T> Callable<T> {
    /// This is the generic return type call
    ///
    pub fn call(self) -> T
        where T: Debug,
    {
        println!("res={:?}", self.res);
        self.res
        //unimplemented!()
    }
}

/// This will be our RequestBuilder thingy
/// 
#[derive(Debug, Copy, Clone)]
struct Request<T> {
    val: T,
}

/// And how we implement both `get()` and `list()`
///
impl<T> Request<T> {
    fn new(v: T) -> Self {
        Self { val: v }
    }

    fn get(self) -> Callable<T>
        where T: Copy,
    {
        Callable { res: self.val }
    }

    fn list(self) -> Callable<Vec<T>>
        where T: Copy,
    {
        Callable { res: vec![self.val, self.val ] }
    }
}

/// Practical example.
///
fn main() -> Result<()> {
    let a = Request::new(5);
    let b = Request::new("aa");

    let r = a.get().call();
    let s = b.list().call();

    Ok(())
}
