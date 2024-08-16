use regex::Regex;
use reqwest;
use scraper::ElementRef;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fmt::format;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The URL you want to fetch
    let url = "https://www.olx.ro/imobiliare/apartamente-garsoniere-de-inchiriat/cluj-napoca/?currency=EUR";
    let client = reqwest::Client::new();

    // Send a GET request and wait for the response
    let response = client.get(url).send().await?;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("content.txt")
        .unwrap();
    // Check if the request was successful
    if response.status().is_success() {
        // Get the response text (HTML content)
        let target_class = "css-1venxj6";
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let format = format(format_args!("div.{}", target_class));
        let selector = Selector::parse(&format).unwrap();
        let matching_divs: Vec<_> = document.select(&selector).collect();
        if matching_divs.is_empty() {
            print!("No matching divs found");
        } else {
            let mut vec = Vec::new();

            for (index, content) in matching_divs.iter().enumerate() {
                let apData = AppartmentData::from_div(content);
                vec.push(apData);
            }
            let (av1, av2, ratio) = calculate_averages(&vec[..]);
            println!("Average of field1: {:.2}", av1);
            println!("Average of field2: {:.2}", av2);
            println!("Ratio of averages (field1 / field2): {:.2}", ratio);
        }
        // Alternatively, you could save it to a file
        // std::fs::write("page.html", &html_content)?;
    } else {
        println!("Failed to get page: HTTP {}", response.status());
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct AppartmentData {
    title: String,
    surface: u16,
    price: u16,
}
impl AppartmentData {
    fn from_div(element: &ElementRef) -> Self {
        let title_class = "css-1wxaaza";
        let price_class = "css-13afqrm";
        let surface_class = "css-643j0o";
        let format_h = format(format_args!("h6.{}", title_class));
        let selector = Selector::parse(&format_h).unwrap();
        let matching_divs: Vec<_> = element.select(&selector).collect();
        let mut title = String::new();
        if matching_divs.is_empty() {
            print!("No divs found for title");
        } else if matching_divs.len() > 1 {
            eprint!("Found multiple divs with title class");
        } else {
            for (_i, content) in matching_divs.iter().enumerate() {
                title = content.inner_html();
            }
        }
        let mut price: u16 = 0;
        let format_p = format(format_args!("p.{}", price_class));
        let selector = Selector::parse(&format_p).unwrap();
        let matching_divs: Vec<_> = element.select(&selector).collect();
        let trim_char = '€';
        if matching_divs.is_empty() {
            print!("No divs found for title");
        } else if matching_divs.len() > 1 {
            eprint!("Found multiple divs with title class");
        } else {
            for (_i, content) in matching_divs.iter().enumerate() {
                let content = content.inner_html();
                let content = content.split(trim_char).next().unwrap_or(&content);
                price = content
                    .replace("€", "")
                    .replace(" ", "")
                    .trim()
                    .parse()
                    .expect(format!("Unable to parse the price {}", content).as_str());
            }
        }
        let mut surface: u16 = 0;
        let format_p = format(format_args!("span.{}", surface_class));
        let selector = Selector::parse(&format_p).unwrap();
        let matching_divs: Vec<_> = element.select(&selector).collect();
        if matching_divs.is_empty() {
            print!("No divs found for title");
        } else if matching_divs.len() > 1 {
            eprint!("Found multiple divs with title class");
        } else {
            for (_i, content) in matching_divs.iter().enumerate() {
                let content = content.inner_html();
                let regex = Regex::new("\\d+ m²").unwrap();
                let is_match = regex.find(&content);
                if let Some(cap) = is_match {
                    surface = cap
                        .as_str()
                        .replace(" m²", "")
                        .trim()
                        .parse()
                        .expect(format!("Unable to parse the surface {}", content).as_str());
                }
            }
        }
        Self {
            title,
            surface,
            price,
        }
    }
}

fn calculate_averages(data: &[AppartmentData]) -> (f64, f64, f64) {
    if data.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    let count = data.len() as f64;
    let (sum1, sum2) = data.iter().fold((0u32, 0u32), |(acc1, acc2), item| {
        (acc1 + item.price as u32, acc2 + item.surface as u32)
    });

    let avg1 = sum1 as f64 / count;
    let avg2 = sum2 as f64 / count;

    // Avoid division by zero
    let ratio = if avg2 != 0.0 {
        avg1 / avg2
    } else {
        f64::INFINITY
    };

    (avg1, avg2, ratio)
}
