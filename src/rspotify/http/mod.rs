use serde_json::Value;
use std::collections::HashMap;
use ureq::{Agent, Error};

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Form<'a> = HashMap<&'a str, &'a str>;

#[derive(Debug, Clone)]
pub struct HttpClient {
    agent: Agent,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            agent: Agent::new_with_defaults(),
        }
    }
}

impl HttpClient {
    pub fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String, Error> {
        let mut request = self.agent.get(url);
        if let Some(headers) = headers {
            for (key, val) in headers.iter() {
                request = request.header(key, val);
            }
        }
        for (key, val) in payload.iter() {
            request = request.query(key, val);
        }
        match request.call() {
            Ok(response) => response.into_body().read_to_string(),
            Err(err) => Err(err),
        }
    }

    pub fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Error> {
        let mut request = self.agent.post(url);
        if let Some(headers) = headers {
            for (key, val) in headers.iter() {
                request = request.header(key, val);
            }
        }
        match request.send_json(payload) {
            Ok(response) => response.into_body().read_to_string(),
            Err(err) => Err(err),
        }
    }

    pub fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'_>,
    ) -> Result<String, Error> {
        let mut request = self.agent.post(url);
        if let Some(headers) = headers {
            for (key, val) in headers.iter() {
                request = request.header(key, val);
            }
        }
        let payload = payload
            .iter()
            .map(|(key, val)| (*key, *val))
            .collect::<Vec<_>>();
        match request.send_form(payload) {
            Ok(response) => response.into_body().read_to_string(),
            Err(err) => Err(err),
        }
    }

    pub fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Error> {
        let mut request = self.agent.put(url);
        if let Some(headers) = headers {
            for (key, val) in headers.iter() {
                request = request.header(key, val);
            }
        }
        match request.send_json(payload.clone()) {
            Ok(response) => response.into_body().read_to_string(),
            Err(err) => Err(err),
        }
    }

    pub fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Error> {
        let mut request = self.agent.delete(url).force_send_body();
        if let Some(headers) = headers {
            for (key, val) in headers.iter() {
                request = request.header(key, val);
            }
        }
        match request.send_json(payload.clone()) {
            Ok(response) => response.into_body().read_to_string(),
            Err(err) => Err(err),
        }
    }
}
