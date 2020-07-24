//use crossbeam::thread;
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::marker::{Send, Sync};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, sleep};
use std::time::Duration;

#[derive(Debug)]
pub struct Beat<T>
where
    T: Serialize + Send + Sync,
{
    index: String,
    timestamp: Option<String>,
    //    mappings: Option<HashMap<String, String>>,
    data: Arc<RwLock<T>>,
    client: Client,
}

impl<T> Beat<T>
where
    T: Serialize + Send + Sync,
{
    pub fn new(
        index: &str,
        data: Arc<RwLock<T>>,
        timestamp: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let b = Beat {
            index: index.to_owned(),
            //            mappings: None,
            data: data,
            client: Client::new(),
            timestamp: timestamp,
        };
        Ok(b)
    }

    pub fn send(&self) -> Result<String, Box<dyn Error>> {
        let d = self.data.read().expect("RwLock is poisned");
        let j: Vec<u8>;
        match &self.timestamp {
            Some(t) => j = self.add_timestamp(t)?.as_bytes().to_vec(),
            None => j = serde_json::to_vec(&*d)?,
        }
        let response = self
            .client
            .post(&self.index)
            .body(j)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .send()?;
        Ok(response.text()?)
    }

    pub fn add_timestamp(&self, time_format: &str) -> Result<String, Box<dyn Error>> {
        let d = self.data.read().expect("Rwlock is poisned");
        let mut j = serde_json::to_string(&*d)?;
        let timestamp: DateTime<Utc> = Utc::now();
        j.pop();
        j.push_str(", \"TimeStamp\": \"");
        j.push_str(format!("{}", timestamp.format(time_format)).as_str());
        j.push_str("\"}");
        Ok(j)
    }
}

pub fn monitor<T>(
    data: Arc<RwLock<T>>,
    freq: Duration,
    index: &str,
    timestamp: Option<String>,
    mut times: u8,
) -> thread::JoinHandle<()>
where
    T: Serialize + Send + Sync + 'static,
{
    let beat = Beat::new(&index, data, timestamp).unwrap();
    thread::spawn(move || {
        while times > 0 {
            thread::sleep(freq);
            let resp = beat.send();
            println!("{}", resp.unwrap());
            times -= 1;
        }
    })
}
