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
    pub fn new() -> Self {
        let name = env!("CARGO_PKG_NAME");
        let author = env!("CARGO_PKG_AUTHORS");
        let version = env!("CARGO_PKG_VERSION");
        let license = env!("CARGO_PKG_LICENSE");
        let repo = env!("CARGO_PKG_REPOSITORY");
        let msrv = env!("CARGO_PKG_RUST_VERSION");

        let logo = include_str!("../../assets/uefi.logo");

        Self {
            name,
            author,
            version,
            license,
            repo,
            msrv,
            logo,
        }
    }
}
