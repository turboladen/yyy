use std::{
    env,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use anyhow::bail;
use config::{Config, Environment, File};
use serde::Deserialize;

const DEFAULTS_FILE: &str = "config/defaults";

#[derive(Debug)]
pub(crate) struct Settings {
    database: Database,
    tcp: Tcp,
}

impl Settings {
    pub(crate) fn try_new() -> anyhow::Result<Self> {
        let yyy_env = match env::var("YYY_ENV") {
            Err(_) => Env::Dev,
            Ok(s) => match s.as_str() {
                "dev" => Env::Dev,
                "test" => Env::Test,
                t => bail!("Unknown environment: {t}"),
            },
        };

        let config_file_path = match yyy_env {
            Env::Dev => "config/dev",
            Env::Test => "config/test",
        };

        let s = Config::builder()
            .add_source(File::with_name(DEFAULTS_FILE))
            .add_source(File::with_name(config_file_path))
            .add_source(Environment::with_prefix("yyy"))
            .build()?;

        let s: SettingsFile = s.try_deserialize()?;

        Ok(Self {
            database: s.database,
            tcp: s.tcp,
        })
    }

    pub(crate) const fn database(&self) -> &Database {
        &self.database
    }

    pub(crate) const fn tcp(&self) -> &Tcp {
        &self.tcp
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Database {
    file: String,
    namespace: String,
    name: String,
}

impl Database {
    pub(crate) fn file(&self) -> &str {
        self.file.as_ref()
    }

    pub(crate) fn namespace(&self) -> &str {
        self.namespace.as_ref()
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Tcp {
    host: String,
    port: u16,
}

impl Tcp {
    pub(crate) fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        Ok(SocketAddrV4::new(self.host.parse::<Ipv4Addr>()?, self.port).into())
    }
}

#[derive(Debug)]
pub(crate) enum Env {
    Dev,
    Test,
}

#[derive(Debug, Deserialize)]
struct SettingsFile {
    database: Database,
    tcp: Tcp,
}
