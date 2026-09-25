pub mod cdp;
pub mod discovery;
pub mod scripts;
pub mod session;

pub use cdp::{CdpClient, TargetInfo};
pub use discovery::{BrowserEndpoint, discover};
