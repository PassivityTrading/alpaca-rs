use async_tungstenite::WebSocketStream;
use reqwest::Url;

pub struct LiveClient<S> {
    pub socket: WebSocketStream<S>,
    pub base_url: Url,
}
