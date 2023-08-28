use sm::{Mpv, Res, Sm, G};

fn main() -> Res<()> {
    let g = G {
        repo: sm::Repo::Github {
            url: format!("https://github.com/rsuu/sm"),
        },
    };

    let mpv = Mpv::new(&g);
    mpv.dl()?;

    Ok(())
}
