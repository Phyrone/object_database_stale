use log::trace;

pub mod serializer;
pub mod string_parser;

pub const VERSION: &str = CARGO_PKG_VERSION;

pub const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const CARGO_PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const CARGO_PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const CARGO_PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CARGO_PKG_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const CARGO_PKG_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const CARGO_PKG_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
pub const CARGO_PKG_VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");

pub async fn log_const() {
    trace!("Cargo:");
    trace!("  CARGO_PKG_VERSION: {}", CARGO_PKG_VERSION);
    trace!("  CARGO_PKG_VERSION_MAJOR: {}", CARGO_PKG_VERSION_MAJOR);
    trace!("  CARGO_PKG_VERSION_MINOR: {}", CARGO_PKG_VERSION_MINOR);
    trace!("  CARGO_PKG_VERSION_PATCH: {}", CARGO_PKG_VERSION_PATCH);
    trace!("  CARGO_PKG_VERSION_PRE: {}", CARGO_PKG_VERSION_PRE);
    trace!("  CARGO_PKG_AUTHORS: {}", CARGO_PKG_AUTHORS);
    trace!("  CARGO_PKG_DESCRIPTION: {}", CARGO_PKG_DESCRIPTION);
    trace!("  CARGO_PKG_HOMEPAGE: {}", CARGO_PKG_HOMEPAGE);
    trace!("  CARGO_PKG_NAME: {}", CARGO_PKG_NAME);
    trace!("  CARGO_PKG_REPOSITORY: {}", CARGO_PKG_REPOSITORY);
}
