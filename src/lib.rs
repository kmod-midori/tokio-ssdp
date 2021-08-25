//! A mininal SSDP device implementation using `tokio`.

mod device;
pub use device::Device;

mod server;
pub use server::Server;

