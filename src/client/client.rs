use crate::{
    models::{
        ExchangeBindings, ExchangeInfo, Overview,
        QueueInfo,
    },
};

use serde::{de::DeserializeOwned, Serialize};

pub trait ManagementClient: Send + Sync {
    fn get_exchange_overview(&self) -> Vec<ExchangeInfo>;
    fn get_exchange_bindings(&self, exch: &ExchangeInfo) -> Vec<ExchangeBindings>;
    fn get_overview(&self) -> Overview;
    fn get_queues_info(&self) -> Vec<QueueInfo>;
    fn ping(&self) -> Result<(), reqwest::Error>;
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
    client: reqwest::blocking::Client,
}


impl Client {
    pub fn new(addr: &str, user: &str, pass: Option<String>) -> Self {
        Self {
            addr: addr.to_string(),
            user: user.to_string(),
            pass,
            client: reqwest::blocking::Client::new(),
        }
    }

    // TODO change this to Result and cover api failures!!
    fn get<T>(&self, endpoint: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.addr, endpoint);
        self.client
            .get(url)
            .basic_auth(&self.user, self.pass.as_ref())
            .send()?
            .json()
    }
}

impl ManagementClient for Client {
    fn get_exchange_overview(&self) -> Vec<ExchangeInfo> {
        self.get::<Vec<ExchangeInfo>>("/api/exchanges").unwrap()
    }

    #[allow(dead_code)]
    fn get_exchange_bindings(&self, exch: &ExchangeInfo) -> Vec<ExchangeBindings> {
        let n = exch.vhost.replace("/", "%2F");
        let endpoint = format!("/api/exchanges/{}/{}/bindings/source", n, exch.name);
        self.get::<Vec<ExchangeBindings>>(&endpoint).unwrap()
    }

    #[allow(dead_code)]
    fn get_overview(&self) -> Overview {
        self.get::<Overview>("/api/overview").unwrap()
    }

    fn get_queues_info(&self) -> Vec<QueueInfo> {
        self.get::<Vec<QueueInfo>>("/api/queues").unwrap()
    }

    fn ping(&self) -> Result<(), reqwest::Error> {
        // TODO better ping?
        match self.get::<Overview>("/api/overview") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
