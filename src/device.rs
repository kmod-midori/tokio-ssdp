use log::debug;

/// Information about a SSDP device or service.
#[derive(Debug, Clone)]
pub struct Device {
    pub(crate) usn: String,
    pub(crate) search_target: String,
    pub(crate) location: String,
}

impl Device {
    /// Create a new SSDP device or service.
    ///
    /// # Examples
    /// ```
    /// # use tokio_ssdp::Device;
    /// let uuid = "ad8782a0-9e28-422b-a6ae-670fe7c4c043";
    /// // uuid:{}::upnp:rootdevice
    /// Device::new(uuid, "upnp:rootdevice", "http://192.168.1.100:8080/desc.xml");
    /// // uuid:{}
    /// Device::new(uuid, "", "http://192.168.1.100:8080/desc.xml");
    /// // uuid:{}::urn:schemas-upnp-org:device:MediaRenderer:1
    /// Device::new(uuid, "urn:schemas-upnp-org:device:MediaRenderer:1", "http://192.168.1.100:8080/desc.xml");
    /// ```
    pub fn new(
        uuid: impl AsRef<str>,
        search_target: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        let st: String = search_target.into();

        let usn = if st.is_empty() {
            format!("uuid:{}", uuid.as_ref())
        } else {
            format!("uuid:{}::{}", uuid.as_ref(), st)
        };

        debug!("USN: {}", usn);

        Self {
            usn,
            search_target: st,
            location: location.into(),
        }
    }
}
