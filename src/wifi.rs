use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::modem::Modem,
    nvs::EspDefaultNvsPartition,
    sys::EspError,
    wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};

pub fn init_wifi<'a>(modem: Modem) -> Result<BlockingWifi<EspWifi<'a>>, EspError> {
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let raw_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;

    let mut wifi = BlockingWifi::wrap(raw_wifi, sysloop).inspect_err(|e| {
        log::error!("Failed to wrap wifi in blocking mode: {:?}", e);
    })?;

    log::info!("Setting wifi configuration to client mode");
    if let Err(e) = wifi.set_configuration(&Configuration::Client(ClientConfiguration::default())) {
        log::error!("Failed to set wifi configuration: {:?}", e);
        return Err(e);
    };

    if let Err(e) = wifi.start() {
        log::error!("Failed to start wifi: {:?}", e);
        return Err(e);
    };

    Ok(wifi)
}
