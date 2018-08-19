use grpcio::{Channel, ChannelBuilder, EnvBuilder};

use std::sync::Arc;

pub struct Client {
    ch: Channel,
}

impl Client {
    pub fn new<S>(host: S, port: u64) -> Client
    where
        S: Into<String>,
    {
        let addr = format!("{}:{}", host.into(), port);
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env).connect(&addr);

        Client { ch: channel }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
