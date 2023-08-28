//! MPV

// git clone SRC
// cp -ru SRC DST
// rm SRC

use crate::*;

use futures_lite::future;

#[derive(Debug)]
pub struct Mpv {
    repo: Repo,
    sh: Shell,
}

impl Mpv {
    async fn inner_dl(&self) -> Res<()> {
        let url = self.repo.get_url();

        cmd!(self.sh, "git clone --depth 1 {url}").quiet().run()?;

        Ok(())
    }
}

impl Sm for Mpv {
    fn new(g: &G) -> Self {
        Self {
            repo: g.repo.clone(),
            sh: Shell::new().unwrap(),
        }
    }

    fn dl(&self) -> Res<()> {
        block_on(async {
            self.inner_dl().await.unwrap();
        });

        Ok(())
    }
}
