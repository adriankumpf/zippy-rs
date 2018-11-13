use meval;
use rayon::prelude::*;
use reqwest;

use std::fs::File;

fn main() -> Result<(), ()> {
    std::env::args()
        .collect::<Vec<String>>()
        .par_iter()
        .for_each(|url| {
            if url.contains("zippyshare") {
                let url = get_file_url(&url).unwrap();
                download_file(&url).unwrap()
            }
        });

    Ok(())
}

fn get_file_url(url: &str) -> Result<String, &str> {
    let body = reqwest::get(url).unwrap().text().unwrap();

    let prefix = "getElementById('dlbutton').href = \"";
    let suffix = "\";";

    let prefix_start = body.find(prefix).unwrap() + prefix.len();
    let trimmed = &body[prefix_start..body.len()];

    let suffix_end = trimmed.find(suffix).unwrap();
    let trimmed = &trimmed[0..suffix_end];

    if let [p1, formula, p2] = trimmed.split("\"").collect::<Vec<&str>>().as_slice() {
        let formula = formula.trim_start_matches(" + ").trim_end_matches(" + ");
        let result = meval::eval_str(formula).unwrap();

        let end = url.find(".com").unwrap() + ".com".len();
        let base_url = &url[0..end];

        return Ok(format!("{}{}{}{}", base_url, p1, result, p2));
    };

    Err("invalid url")
}

fn download_file(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = url.rsplit("/").next().unwrap();

    println!("Downloading {} ...", file_name);

    let mut res = reqwest::get(url)?;
    let mut file = File::create(file_name)?;

    std::io::copy(&mut res, &mut file)?;

    Ok(())
}
