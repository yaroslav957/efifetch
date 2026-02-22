use crate::{error::Result, info::InfoItem};
use heapless::String;

#[derive(Clone)]
pub struct Env {
    pub name: String<8>,
    pub author: String<24>,
    pub version: String<8>,
    pub license: String<8>,
    pub repo: String<32>,
    pub msrv: String<8>,

    pub logo: &'static str,
}

impl Env {
    pub fn new() -> Result<Self> {
        let name = {
            let mut s = String::new();
            s.push_str(env!("CARGO_PKG_NAME"))?;
            s
        };
        let author = {
            let mut s = String::new();
            s.push_str(env!("CARGO_PKG_AUTHORS"))?;
            s
        };
        let version = {
            let mut s = String::new();
            s.push_str(env!("CARGO_PKG_VERSION"))?;
            s
        };
        let license = {
            let mut s = String::new();
            s.push_str(env!("CARGO_PKG_LICENSE"))?;
            s
        };
        let repo = {
            let mut s = String::new();
            s.push_str(env!("CARGO_PKG_REPOSITORY"))?;
            s
        };
        let msrv = {
            let mut s = String::new();
            s.push_str(env!("CARGO_PKG_RUST_VERSION"))?;
            s
        };

        let logo = include_str!("../../assets/uefi.logo");

        Ok(Self {
            name,
            author,
            version,
            license,
            repo,
            msrv,
            logo,
        })
    }
}

impl InfoItem for Env {
    fn render(&self) -> impl Iterator<Item = (&str, &str)> {
        [
            ("Binary:", self.name.as_str()),
            ("Author:", self.author.as_str()),
            ("Version:", self.version.as_str()),
            ("License:", self.license.as_str()),
            ("Repo:", self.repo.as_str()),
            ("MSRV:", self.msrv.as_str()),
        ]
        .into_iter()
    }
}
