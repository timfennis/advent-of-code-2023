use download::Downloader;

mod download;

fn main() -> anyhow::Result<()> {
    let downloader = Downloader::from_env()?;
    downloader.day(2022, 22)
}
