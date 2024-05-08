use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};
use toml::de::Error;

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub addr: Addr,
    pub port: Port,
}

impl Config {
    pub fn new(
        addr_public: Ipv4Addr,
        addr_internal: Ipv4Addr,
        port_http: u16,
        port_https: u16,
    ) -> Self {
        Self {
            addr: Addr {
                public: addr_public,
                internal: addr_internal,
            },
            port: Port {
                http: port_http,
                https: port_https,
            },
        }
    }

    pub fn load_from_str(string: &str) -> Result<Self, Error> {
        toml::from_str(string)
    }

    pub fn load_from_embeded() -> Result<Self, Error> {
        toml::from_str(include_str!("../Config.toml"))
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Addr {
    pub public: Ipv4Addr,
    pub internal: Ipv4Addr,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Port {
    pub http: u16,
    pub https: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: Addr {
                public: Ipv4Addr::new(0, 0, 0, 0),
                internal: Ipv4Addr::new(127, 0, 0, 1),
            },
            port: Port {
                http: 8000,
                https: 8443,
            },
        }
    }
}
