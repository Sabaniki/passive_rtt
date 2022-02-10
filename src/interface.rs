use pnet::datalink::NetworkInterface;
use pnet::datalink;
use anyhow::Context;


pub fn get_from_name(interface_name: String) -> Result<NetworkInterface, anyhow::Error> {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|interface| interface.name == interface_name)
        .with_context(||format!("failed to select the interface '{}'", interface_name))?;
    Ok(interface)
}