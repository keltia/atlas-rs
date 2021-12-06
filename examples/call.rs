use std::fmt::Debug;
use anyhow::Result;

#[derive(Debug)]
struct Callable<T> { res: T }

impl<T> Callable<T> {
    pub fn call(self) -> T
        where T: Debug,
    {

        //println!("res={:?}", self.res)
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone)]
struct Request<T> {
    val: T,
}

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


fn main() -> Result<()> {
    let a = Request::new(5);
    let b = Request::new("aa");

    println!("{:?}", a.get());
    println!("{:?}", a.list());

    println!("{:?}", b.get());
    println!("{:?}", b.list());

    println!("{:?}", a.get().call());

    Ok(())
}
