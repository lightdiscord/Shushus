#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid configuration")]
    InvalidConfiguration,

    #[fail(display = "invalid or missing state")]
    InvalidState,

    #[fail(display = "invalid or missing code")]
    InvalidCode,
}
