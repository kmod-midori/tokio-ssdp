use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio_ssdp::{Device, Server};

const DEVICE_UUID: &str = "a1ab85e9-e299-4005-a427-f7e49cb1e119";

fn unix_ts_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Based on https://github.com/xfangfang/Macast/blob/main/macast/plugin.py
    let server_fut = Server::new([
        // Device
        Device::new(
            DEVICE_UUID,
            "upnp:rootdevice",
            "http://192.168.2.129:4399/desc.xml",
        ),
        Device::new(DEVICE_UUID, "", "http://192.168.2.129:4399/desc.xml"),
        // MediaRenderer
        Device::new(
            DEVICE_UUID,
            "urn:schemas-upnp-org:device:MediaRenderer:1",
            "http://192.168.2.129:4399/desc.xml",
        ),
    ])
    .extra_header("BOOTID.UPNP.ORG", unix_ts_secs().to_string())
    .extra_header("CONFIGID.UPNP.ORG", "1")
    .serve()?;

    tokio::select! {
        _ = server_fut => {}
        _ = tokio::signal::ctrl_c() => {}
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(())
}
