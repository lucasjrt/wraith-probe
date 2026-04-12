#[derive(Debug, PartialEq, Eq)]
pub enum AppEvent {
    DownPressed,
    UpPressed,
    SelectPressed,
    BackPressed,
    Net(NetworkEvent),
    Redraw,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkEvent {
    Connected,
    Disconnected,
    IpAcquired(String),
    ScanStarted,
    ScanCompleted(Vec<String>),
    ScanFailed,
}
