// use std::collections::HashMap;
// use wascc_host::{Actor, Host, NativeCapability};
use wascc_host::{Host, NativeCapability};
use wascc_httpsrv::HttpServerProvider;

// const CAP_OLED_PROVIDER: &str = "red-badger:oled-ssd1306";

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();

    let host = Host::new();

    let http_server_provider = NativeCapability::from_instance(HttpServerProvider::new(), None)?;
    // let cap_http_server_provider = &http_server_provider.descriptor().id.clone();
    host.add_native_capability(http_server_provider)?;

    // let actor = Actor::from_file(
    //     "../wasm_oled_actor/target/wasm32-unknown-unknown/debug/wasm_oled_actor_s.wasm",
    // )?;
    // let actor_pub_key = &actor.public_key();
    // host.add_actor(actor)?;

    // host.set_binding(
    //     actor_pub_key,
    //     cap_http_server_provider,
    //     None,
    //     generate_port_config(8081),
    // )?;
    // host.set_binding(actor_pub_key, CAP_OLED_PROVIDER, None, HashMap::new())?;

    std::thread::park();

    Ok(())
}

// fn generate_port_config(port: u16) -> HashMap<String, String> {
//     let mut hm = HashMap::new();
//     hm.insert("PORT".to_string(), port.to_string());

//     hm
// }
