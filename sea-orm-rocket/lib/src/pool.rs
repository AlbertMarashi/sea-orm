use rocket::figment::Figment;

/// Generic [`Database`](crate::Database) driver connection pool trait.
///
/// This trait provides a generic interface to various database pooling
/// implementations in the Rust ecosystem. It can be implemented by anyone, but
/// this crate provides implementations for common drivers.
/// ```
#[rocket::async_trait]
pub trait Pool: Sized + Send + Sync + 'static {
    /// The connection type managed by this pool, returned by [`Self::get()`].
    type Connection;

    /// The error type returned by [`Self::init()`] and [`Self::get()`].
    type Error: std::error::Error;

    /// Constructs a pool from a [Value](rocket::figment::value::Value).
    ///
    /// It is up to each implementor of `Pool` to define its accepted
    /// configuration value(s) via the `Config` associated type.  Most
    /// integrations provided in `sea_orm_rocket` use [`Config`], which
    /// accepts a (required) `url` and an (optional) `pool_size`.
    ///
    /// ## Errors
    ///
    /// This method returns an error if the configuration is not compatible, or
    /// if creating a pool failed due to an unavailable database server,
    /// insufficient resources, or another database-specific error.
    async fn init(figment: &Figment) -> Result<Self, Self::Error>;

    /// Borrow the inner connection
    fn borrow(&self) -> &Self::Connection;
}

#[derive(Debug)]
/// A mock object which impl `Pool`, for testing only
pub struct MockPool;

#[derive(Debug)]
pub struct MockPoolErr;

impl std::fmt::Display for MockPoolErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MockPoolErr {}

#[rocket::async_trait]
impl Pool for MockPool {
    type Error = MockPoolErr;

    type Connection = bool;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        Ok(MockPool)
    }

    fn borrow(&self) -> &Self::Connection {
        &true
    }
}
