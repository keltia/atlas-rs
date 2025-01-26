//! Macros

/// The `action_keyword` macro is a convenient utility for generating methods associated with
/// API operations. This macro reduces redundancy by automating the creation of methods and their
/// corresponding documentation.
///
/// # Arguments
///
/// The macro accepts the following mandatory arguments:
///
/// - `name`: The name of the method being generated (e.g., `get`, `list`).
/// - `op`: The specific operation associated with the method, represented by a variant of the `Op` enum (e.g., `Op::List`).
/// - `ret`: The return type of the method, which can be either `Single` or `Paged`.
///
/// Additionally, if a `data` argument is present, it also generates a method that takes a single
/// parameter of type `Param`.
///
/// # Examples
///
/// ## Without parameters
///
/// ```no_run
/// # use atlas_api::client::ClientBuilder;
/// # use atlas_api::core::probes::Probe;
/// # use atlas_api::errors::APIError;
/// # use atlas_api::request::*;
///
/// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
/// let res: Result<Return<Probe>, APIError> = c.probe().get().call();
/// ```
///
/// ## With a parameter
///
/// ```no_run
/// # use atlas_api::client::ClientBuilder;
/// # use atlas_api::core::probes::Probe;
/// # use atlas_api::errors::APIError;
/// # use atlas_api::param::Param;
/// # use atlas_api::request::*;
///
/// let data = Param::None;
/// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
/// let res: Result<Return<Probe>, APIError> = c.probe().filter(data).call();
/// ```
///
/// This macro abstracts away repetitive code patterns, simplifying the process of building
/// consistent and well-documented API clients.
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
    /// # use atlas_api::errors::APIError;use atlas_api::param::Param;
    /// # use atlas_api::request::*;
    ///
    /// # let data = Param::None;
    /// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();    ///
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
