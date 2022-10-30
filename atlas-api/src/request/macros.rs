/// Define the `action_keyword` macro that generate the code and documentation for our first
/// action calls like `get()` and `list()`.  That way we don't have to repeat everything.
///
/// It takes the following mandatory arguments:
/// - `name` is the name of the call
/// - `op`  is the member of the `Op` enum like `Op::List`  without the prefix
/// - `ret` is the return type, either `Single` or `Paged`
///
/// If `data` is also present, it will insert the definition of a single parameter of type `Param`.
///
#[macro_export]
macro_rules! action_keyword {
    ($name:ident, $op:ident, $ret:ty) => {
    #[doc = concat!("This is the `", stringify!($name), "()` method for `", stringify!($ret), "`")]
    /// results and no parameter.
    ///
    /// ```no_run
    /// # use atlas_api::client::ClientBuilder;
    /// # use atlas_api::core::probes::Probe;
    /// # use atlas_api::errors::APIError;
    /// # use atlas_api::request::*;
    ///
    /// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
    ///
    #[doc = concat!("let res: Result<Return<Probe>, APIError> = c.probe().", stringify!($name), "().call();")]
    ///
    /// ```
    pub fn $name(self) -> $ret {
        let mut req = <$ret>::from(self);
        req.op = Op::$op;
        req
    }};
    ($name:ident, $op:ident, $ret:ty, $data:ident) => {
    #[doc = concat!("This is the `", stringify!($name), "()` method for `", stringify!($ret), "`")]
    /// results and a parameter(see [`Param`]).
    ///
    /// ```no_run
    /// # use atlas_api::client::ClientBuilder;
    /// # use atlas_api::core::probes::Probe;
    /// # use atlas_api::errors::APIError;
    /// # use atlas_api::request::*;
    ///
    /// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
    ///
    #[doc = concat!("let res: Result<Return<Probe>, APIError> = c.probe().", stringify!($name), "(", stringify!($data), ").call();")]
    ///
    /// ```

    pub fn $name<P>(self, $data: P) -> $ret
        where
            P: Into<Param> + Debug,
    {
        let mut req = <$ret>::from(self);
        req.query = $data.into();
        req.op = Op::$op;
        req
    }};
}

