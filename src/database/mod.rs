use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Service {
  name: String,
  host: String,
  ipaddr: String,
  #[serde(default = "default_ttl")]
  ttl: u32,
  #[serde(default)]
  healthcheck: String,
  #[serde(default = "default_http_ok")]
  healthcheck_code: u32,
}

lazy_static!{
  static ref SERVICES: Mutex<HashMap<String, Service>> = Mutex::new(HashMap::new());
}

pub fn update(svc: &Service) -> &Service {
  SERVICES.lock().unwrap().insert(String::from(svc.host.clone()), svc.clone());
  return svc;
}

pub fn list() -> Vec<Service> {
  let mut vec = Vec::new();
  for (_, value) in &*(SERVICES.lock().unwrap()) {
    vec.push(value.clone());
  }
  return vec;
}

fn default_http_ok() -> u32 {
  return 200;
}

fn default_ttl() -> u32 {
  return 300;
}
