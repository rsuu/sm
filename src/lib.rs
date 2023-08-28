//! # sm
//!
//! Script Manager
//!
//! ## Usage
//!
//! ```bash
//! sm mpv dl xxx.git
//!
//! sm mpv sync
//! sm mpv sync -p abc
//! sm mpv sync -p abcdef
//!
//! sm mpv rm -p abc
//! sm mpv rm --all
//! ```

mod mpv;

pub use mpv::Mpv;

pub use {
    futures_lite::future::block_on,
    tempfile::{tempdir, tempfile},
    xshell::{cmd, Shell},
};

pub type Res<T> = anyhow::Result<T>;

#[derive(Debug, Clone)]
pub enum Repo {
    Github { url: String },
    Gist { url: String },
}

impl Repo {
    fn get_url(&self) -> &str {
        match self {
            Self::Github { url } => url,
            Self::Gist { url } => url,
        }
    }
}

#[derive(Debug)]
pub struct G {
    pub repo: Repo,
}

pub trait Sm {
    fn new(g: &G) -> Self;

    /// download
    fn dl(&self) -> Res<()> {
        unimplemented!()
    }

    /// sync
    ///
    /// ```text
    /// let tmp = vec![];
    /// for f in script.iter() {
    ///    if f.need_update() {
    ///        tmp.push(
    ///            // Fn
    ///            update()
    ///        )
    ///    }
    /// }
    ///
    /// 'async join_all!{ update() }
    /// ```
    fn sync(&self) -> Res<()> {
        unimplemented!()
    }

    /// remove
    fn rm(&self) -> Res<()> {
        unimplemented!()
    }
}
