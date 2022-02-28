use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message, Result},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

type SplitSocket = (
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
);

pub struct WsClient {
    url: Url,
    socket: Option<SplitSocket>,
}

impl WsClient {
    pub fn new(url: Url) -> Self {
        Self { url, socket: None }
    }

    pub async fn connect(&mut self) -> Result<()> {
        if self.socket.is_some() {
            self.disconnect().await?;
        }

        let (socket, _) = connect_async(&self.url).await.expect("Failed to connect");
        self.socket = Some(socket.split());
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some((write, read)) = self.socket.take() {
            write.reunite(read).unwrap().close(None).await?;
        }
        Ok(())
    }

    pub async fn send(&mut self, message: Message) -> Result<()> {
        if let Some((write, _)) = &mut self.socket {
            write.send(message).await
        } else {
            Err(Error::ConnectionClosed)
        }
    }

    pub async fn receive(&mut self) -> Option<Result<Message>> {
        if let Some((_, read)) = &mut self.socket {
            read.next().await
        } else {
            None
        }
    }
}
