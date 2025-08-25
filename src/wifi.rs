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

// pub fn get_wifi_results_menu<S: InputPin, B: InputPin>(
//     results: Vec<AccessPointInfo>,
// ) -> Result<Menu<S, B>, EspError> {
//     if results.is_empty() {
//         log::warn!("At least one access point is required to display the menu.");
//         return Err(EspError::from(ESP_FAIL).unwrap());
//     }
//
//     let items: Vec<_> = results
//         .into_iter()
//         .map(|ap| MenuItem {
//             label: &ap.ssid,
//             action: MenuAction::Callback(Box::new(move |ctx| {
//                 let mut display = ctx.display.lock().unwrap();
//                 display.text(&format!("Selected: {}", ap.ssid), 10, 10);
//                 display.text(&format!("Signal: {}", ap.signal_strength), 10, 30);
//             })),
//         })
//         .collect();
//
//     Ok(Menu {
//         title: "Available Access Points",
//         items: &items,
//     })
// }
