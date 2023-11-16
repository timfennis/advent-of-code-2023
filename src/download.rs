use anyhow::Context;
use dotenv::dotenv;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Downloader {
    cookie: String,
}

impl Downloader {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        let cookie = std::env::var("SESSION_COOKIE").context("unable to read SESSION_COOKIE")?;
        Ok(Self { cookie })
    }

    pub fn today(&self) -> anyhow::Result<()> {
        let foo = time::OffsetDateTime::now_local().context("unable to get current day")?;
        let year = foo.year();
        let day = foo.day();

        self.day(year, day)
    }

    pub fn day(&self, year: i32, day: u8) -> anyhow::Result<()> {
        let path_string = format!("input/{year}/{day}");
        let path = Path::new(&path_string);

        if path.exists() {
            return Ok(());
        }

        std::fs::create_dir_all(path.parent().context("cannot get parent path")?)
            .context("unable to create directory")?;

        let mut file = File::create(path).context("unable to create file")?;

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header("Cookie", &self.cookie)
            .header("User-Agent", "aoc2023/0.1 fennis.tim@gmail.com")
            .send()
            .context("failed to fetch input from adventofcode")?;

        let input = response
            .text()
            .context("failed to get text from response body")?;

        file.write_all(input.as_bytes())
            .context("unable to write to file")?;

        Ok(())
    }
}
