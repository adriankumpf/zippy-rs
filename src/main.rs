#![feature(try_trait)]

mod err;

use meval;
use rayon::prelude::*;
use reqwest;
use structopt::StructOpt;

use std::fs::File;

use self::err::{Result, ZippydError};

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Number of concurrent downloads
    #[structopt(short = "c")]
    concurrency: Option<usize>,

    /// Zippyshare URL(s)
    #[structopt(name = "URLs")]
    urls: Vec<String>,
}

fn get_file_url(url: &str) -> Result<String> {
    if !url.contains("zippyshare") {
        return Err(ZippydError::InvalidUrl(url.to_string()));
    }

    let body = reqwest::get(url)?.text()?;

    let prefix = "getElementById('dlbutton').href = \"";
    let suffix = "\";";

    let prefix_start = body.find(prefix)? + prefix.len();
    let trimmed = &body[prefix_start..body.len()];

    let suffix_end = trimmed.find(suffix)?;
    let trimmed = &trimmed[0..suffix_end];

    if let [p1, formula, p2] = trimmed.split("\"").collect::<Vec<&str>>().as_slice() {
        let formula = formula.trim_start_matches(" + ").trim_end_matches(" + ");
        let result = meval::eval_str(formula)?;

        let end = url.find(".com")? + ".com".len();
        let base_url = &url[0..end];

        return Ok(format!("{}{}{}{}", base_url, p1, result, p2));
    };

    Err(ZippydError::InvalidUrl(trimmed.to_string()))
}

fn download_file(url: &str) -> Result {
    let file_name = url.rsplit("/").next()?;

    println!("Downloading {} ...", file_name);

    let mut res = reqwest::get(url)?;
    let mut file = File::create(file_name)?;

    std::io::copy(&mut res, &mut file)?;

    Ok(())
}

fn main() -> Result {
    let opt = Opt::from_args();

    if let Some(c) = opt.concurrency {
        rayon::ThreadPoolBuilder::new()
            .num_threads(c)
            .build_global()?;
    }

    opt.urls.par_iter().try_for_each(|url| {
        let url = get_file_url(&url)?;
        download_file(&url)
    })
}
