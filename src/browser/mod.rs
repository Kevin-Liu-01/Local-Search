pub mod cdp;
pub mod discovery;
pub mod scripts;

pub use cdp::{CdpClient, TargetInfo};
pub use discovery::{BrowserEndpoint, discover};
