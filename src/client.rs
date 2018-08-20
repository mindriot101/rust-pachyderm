use failure::Error;
use grpcio::{ChannelBuilder, EnvBuilder};
use protos::pfs::*;
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

    pub fn create_repo<S>(&self, name: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        let mut repo = Repo::new();
        repo.set_name(name.into());

        let mut request = CreateRepoRequest::new();
        request.set_repo(repo);

        self.client.create_repo(&request)?;
        Ok(())
    }

    pub fn list_repo(&self) -> Result<Vec<RepoInfo>, Error> {
        let request = ListRepoRequest::new();
        let response = self.client.list_repo(&request)?;
        Ok(response.repo_info.into())
    }

    pub fn delete_repo<S>(&self, name: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        let mut repo = Repo::new();
        repo.set_name(name.into());

        let mut request = DeleteRepoRequest::new();
        request.set_repo(repo);

        self.client.delete_repo(&request)?;
        Ok(())
    }

    pub fn clear_repos(&self) -> Result<(), Error> {
        let repos = self.list_repo()?;
        for repo in repos {
            let name = repo.repo.get_ref().name.clone();
            self.delete_repo(name)?;
        }
        Ok(())
    }

    pub fn inspect_repo<S>(&self, name: S) -> Result<RepoInfo, Error>
    where
        S: Into<String>,
    {
        let mut repo = Repo::new();
        repo.set_name(name.into());

        let mut request = InspectRepoRequest::new();
        request.set_repo(repo);

        self.client.inspect_repo(&request).map_err(From::from)
    }
}
