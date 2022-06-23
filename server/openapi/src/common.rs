#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExtendedEvent {
    #[serde(flatten)]
    event: bilive_danmaku::event::Event,
    timestamp: i64
}
