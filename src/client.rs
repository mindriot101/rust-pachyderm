use failure::Error;
use grpcio::{ChannelBuilder, EnvBuilder};
use protos::pfs::{ListRepoRequest, RepoInfo};
use protos::pfs_grpc::ApiClient;
use std::sync::Arc;

pub struct Client {
    client: ApiClient,
}

impl Client {
    pub fn new<S>(host: S, port: u64) -> Client
    where
        S: Into<String>,
    {
        let addr = format!("{}:{}", host.into(), port);
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env).connect(&addr);
        let rpc_client = ApiClient::new(channel);

        Client { client: rpc_client }
    }

    pub fn list_repo(&self) -> Result<Vec<RepoInfo>, Error> {
        let request = ListRepoRequest::new();
        let response = self.client.list_repo(&request)?;
        Ok(response.repo_info.into())
    }
}
