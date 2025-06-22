use crate::{
    models::{
        ExchangeBindings, ExchangeInfo, Overview,
        QueueInfo,
    },
};

use serde::{de::DeserializeOwned, Serialize};

pub trait ManagementClient: Send + Sync {
    fn get_exchange_overview(&self) -> impl std::future::Future<Output = Vec<ExchangeInfo>> + Send;
    fn get_exchange_bindings(&self, exch: &ExchangeInfo) -> impl std::future::Future<Output = Vec<ExchangeBindings>> + Send;
    fn get_overview(&self) -> impl std::future::Future<Output = Overview> + Send;
    fn get_queues_info(&self) -> impl std::future::Future<Output = Vec<QueueInfo>> + Send;
    fn ping(&self) -> impl std::future::Future<Output = Result<(), reqwest::Error>> + Send;
}

#[allow(dead_code)] // we dont use all variants yet, but we might
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Ackmode {
    AckRequeueTrue,
    AckRequeueFalse,
    RejectRequeueTrue,
    RejectRequeueFalse,
}

#[derive(Debug)]
pub struct Client {
    addr: String,
    user: String,
    pass: Option<String>,
    client: reqwest::Client,
}


impl Client {
    pub fn new(addr: &str, user: &str, pass: Option<String>) -> Self {
        Self {
            addr: addr.to_string(),
            user: user.to_string(),
            pass,
            client: reqwest::Client::new(),
        }
    }

    // TODO change this to Result and cover api failures!!
    async fn get<T>(&self, endpoint: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.addr, endpoint);
        self.client
            .get(url)
            .basic_auth(&self.user, self.pass.as_ref())
            .send()
            .await?
            .json()
            .await
    }
}

impl ManagementClient for Client {
    async fn get_exchange_overview(&self) -> Vec<ExchangeInfo> {
        self.get::<Vec<ExchangeInfo>>("/api/exchanges").await.unwrap()
    }

    #[allow(dead_code)]
    async fn get_exchange_bindings(&self, exch: &ExchangeInfo) -> Vec<ExchangeBindings> {
        let n = exch.vhost.replace("/", "%2F");
        let endpoint = format!("/api/exchanges/{}/{}/bindings/source", n, exch.name);
        self.get::<Vec<ExchangeBindings>>(&endpoint).await.unwrap()
    }

    #[allow(dead_code)]
    async fn get_overview(&self) -> Overview {
        self.get::<Overview>("/api/overview").await.unwrap()
    }

    async fn get_queues_info(&self) -> Vec<QueueInfo> {
        self.get::<Vec<QueueInfo>>("/api/queues").await.unwrap()
    }

    async fn ping(&self) -> Result<(), reqwest::Error> {
        // TODO better ping?
        match self.get::<Overview>("/api/overview").await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
