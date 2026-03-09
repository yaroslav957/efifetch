use crate::{error::Result, info::InfoItem};

#[derive(Clone, Copy)]
pub struct Env {
    pub name: &'static str,
    pub author: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub repo: &'static str,
    pub msrv: &'static str,
    pub logo: &'static str,
}

impl Env {
    pub fn new() -> Result<Self> {
        let name = env!("CARGO_PKG_NAME");
        let author = env!("CARGO_PKG_AUTHORS");
        let version = env!("CARGO_PKG_VERSION");
        let license = env!("CARGO_PKG_LICENSE");
        let repo = env!("CARGO_PKG_REPOSITORY");
        let msrv = env!("CARGO_PKG_RUST_VERSION");
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
            ("Binary:", self.name),
            ("Author:", self.author),
            ("Version:", self.version),
            ("License:", self.license),
            ("Repo:", self.repo),
            ("MSRV:", self.msrv),
        ]
        .into_iter()
    }
}
