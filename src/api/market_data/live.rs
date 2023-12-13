use super::*;
use async_tungstenite::WebSocketStream;

pub struct LiveClient<S> {
    pub socket: WebSocketStream<S>,
    pub base_url: Url,
}
