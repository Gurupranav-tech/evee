mod logger;

pub mod application;
pub mod entry_point;
pub mod event;

pub use application::Application;
pub use entry_point::run;
pub use tracing::{debug, error, info, trace, warn};
