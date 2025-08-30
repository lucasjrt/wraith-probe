// use crate::service::Service;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AppEvent {
    DownPressed,
    UpPressed,
    SelectPressed,
    BackPressed,
    Net(NetworkEvent),
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NetworkEvent {
    Connected,
    Disconnected,
    IpAcquired(String),
    ScanStarted,
    ScanCompleted(Vec<String>),
    ScanFailed,
}
