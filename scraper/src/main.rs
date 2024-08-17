use futures::future::{join, join_all};
use regex::Regex;
use reqwest;
use scraper::ElementRef;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let regions = get_regions().await?;
    let region = &regions[5];
    let futures: Vec<_> = region
        .cities
        .iter()
        .map(|city| write_city_to_file(city.name.as_str()))
        .collect();
    let cities = join_all(futures).await;
    for result in cities {
        if let Err(e) = result {
            eprintln!("Error writing city to file: {}", e);
        }
    }
    Ok(())
}
async fn write_city_to_file(city_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut city: City = City::new(String::from(city_name));
    scrape_city(&mut city).await?;
    if city.apartments.is_empty() {
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&city)?;
    let mut file = File::create(format!("{}.json", city_name))?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
async fn get_regions() -> Result<Vec<Region>, Box<dyn std::error::Error>> {
    let mut content = String::new();
    let file = File::open("regions.json");
    match file {
        Result::Ok(mut file_content) => {
            file_content.read_to_string(&mut content)?;
            let regions: Vec<Region> = serde_json::from_str(&content)?;
            if !regions.is_empty() {
                return Ok(regions);
            }
        }
        _ => (),
    }
    let url = "https://www.olx.ro/sitemap/regions/";
    let mut regions: Vec<Region> = Vec::new();
    let client = reqwest::Client::new();

    // Send a GET request and wait for the response
    let response = client.get(url).send().await?;

    // Check if the request was successful
    if response.status().is_success() {
        //print!("Response OK for {}\r", url);
        // Get the response text (HTML content)
        let target_class = "css-g46msg";
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let format_p = format(format_args!("p.{}", target_class));
        let selector = Selector::parse(&format_p).unwrap();
        let matching_ul: Vec<_> = document.select(&selector).collect();
        if matching_ul.is_empty() {
            println!("No regions found");
            ()
        } else {
            for (i, content) in matching_ul.iter().enumerate() {
                let text = content.inner_html();
                let mut cities: Vec<City> = Vec::new();
                let a_element = content
                    .parent()
                    .and_then(|parent| parent.next_sibling())
                    .and_then(|sibling| sibling.first_child());

                if let Some(ul) = a_element {
                    if let Some(ul_element) = ElementRef::wrap(ul) {
                        for il_node in ul_element.children() {
                            if let Some(il) = ElementRef::wrap(il_node) {
                                if let Some(a_elem) = il.first_child() {
                                    if let Some(a) = ElementRef::wrap(a_elem) {
                                        cities.push(City::new(a.inner_html()));
                                    }
                                }
                            }
                        }
                    }
                }
                regions.push(Region { name: text, cities });
            }
        }
    }
    Ok(regions)
}
async fn scrape_city(city: &mut City) -> Result<(), Box<dyn std::error::Error>> {
    let mut vec: Vec<AppartmentData> = Vec::new();
    let name: String = city.name.clone();
    let url = format!(
        "https://www.olx.ro/imobiliare/apartamente-garsoniere-de-inchiriat/{}/?currency=EUR",
        &name
    );
    let results_number = get_results_number(&url).await?;
    if results_number == 0 {
        print!("No results found for this city {}\r", name);
        return Ok(());
    }
    get_data(&url, &mut vec).await?;
    let mut counter = 2;
    let page_number = get_page_number(&url).await?;
    let mut incremented_url = String::from(&url);
    while counter <= page_number {
        get_data(&incremented_url, &mut vec).await?;
        incremented_url = format!("{}&page={}", &url, counter);
        counter += 1;
    }

    println!("Total number: {} for {}", &vec.len(), name);
    let (av1, av2, ratio) = calculate_averages(&vec[..]);
    println!("Average of field1: {:.2}", av1);
    println!("Average of field2: {:.2}", av2);
    println!("Ratio of averages (field1 / field2): {:.2}", ratio);

    city.apartments = vec;
    Ok(())
}

async fn get_results_number(url: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Send a GET request and wait for the response
    let response = client.get(url).send().await?;

    // Check if the request was successful
    if response.status().is_success() {
        //print!("Response OK for {}\r", url);
        // Get the response text (HTML content)
        let target_class = "css-7ddzao";
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let format = format(format_args!("span.{}", target_class));
        let selector = Selector::parse(&format).unwrap();
        let matching_divs: Vec<_> = document.select(&selector).collect();
        if matching_divs.len() != 1 {
            return Ok(0);
        } else {
            for element in matching_divs {
                let regex = Regex::new("\\d+").unwrap();
                let x = element
                    .first_child()
                    .and_then(|child| ElementRef::wrap(child));

                if let Some(span) = x {
                    let content = span.inner_html();
                    let is_match = regex.find(&content);
                    if let Some(cap) = is_match {
                        return Ok(cap
                            .as_str()
                            .replace(" m²", "")
                            .trim()
                            .parse()
                            .expect(format!("Unable to parse the surface {}", &content).as_str()));
                    }
                }
            }
        }
    }
    return Ok(0);
}
async fn scrape_olx() -> Result<i32, Box<dyn std::error::Error>> {
    let mut vec: Vec<AppartmentData> = Vec::new();

    let url =
        "https://www.olx.ro/imobiliare/apartamente-garsoniere-de-inchiriat/slobozia/?currency=EUR";
    get_data(url, &mut vec).await?;
    let mut counter = 2;
    let page_number = get_page_number(url).await?;
    println!("The query has {} pages\r", page_number);
    let mut incremented_url = String::from(url);
    while counter <= page_number {
        get_data(&incremented_url, &mut vec).await?;
        incremented_url = format!("{}&page={}", &url, counter);
        counter += 1;
    }

    println!("Total number: {}", vec.len());
    let (av1, av2, ratio) = calculate_averages(&vec[..]);
    println!("Average of field1: {:.2}", av1);
    println!("Average of field2: {:.2}", av2);
    println!("Ratio of averages (field1 / field2): {:.2}", ratio);
    Ok(1)
}
async fn get_page_number(url: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Send a GET request and wait for the response
    let response = client.get(url).send().await?;

    // Check if the request was successful
    if response.status().is_success() {
        //print!("Response OK for {}\r", url);
        // Get the response text (HTML content)
        let target_class = "css-1mi714g";
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let format = format(format_args!("a.{}", target_class));
        let selector = Selector::parse(&format).unwrap();
        let matching_ul: Vec<_> = document.select(&selector).collect();
        if matching_ul.is_empty() {
            //print!("No matching divs found");
            return Ok(0);
        } else {
            match matching_ul.iter().enumerate().last() {
                Some(last) => {
                    let content = last.1.inner_html();
                    return Ok(content.parse().expect("Cannot parse"));
                }
                _ => Ok(0),
            }
        }
    } else {
        //println!("Failed to get page: HTTP {}", response.status());
        return Ok(0);
    }
}

async fn get_data(
    url: &str,
    vec: &mut Vec<AppartmentData>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Send a GET request and wait for the response
    let response = client.get(url).send().await?;

    // Check if the request was successful
    if response.status().is_success() {
        //print!("Response OK for {}\r", url);
        // Get the response text (HTML content)
        let target_class = "css-1venxj6";
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let format = format(format_args!("div.{}", target_class));
        let selector = Selector::parse(&format).unwrap();
        let matching_divs: Vec<_> = document.select(&selector).collect();
        if matching_divs.is_empty() {
            //print!("No matching divs found");
            return Ok(false);
        } else {
            for (index, content) in matching_divs.iter().enumerate() {
                let ap_data = AppartmentData::from_div(content);
                match vec.last() {
                    Some(last) => {
                        if (ap_data == *last) {
                            return Ok(false);
                        }
                    }
                    _ => (),
                }
                // println!("Adding: {:#?}\r", ap_data);
                if (ap_data.surface > 15 && ap_data.price > 100) {
                    vec.push(ap_data);
                }
            }
        }
    } else {
        //println!("Failed to get page: HTTP {}", response.status());
        return Ok(false);
    }
    Ok(true)
}

#[derive(Debug, Serialize, PartialEq, Deserialize)]
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
            // print!("No divs found for title");
        } else if matching_divs.len() > 1 {
            // eprint!("Found multiple divs with title class");
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
            // print!("No divs found for title");
        } else if matching_divs.len() > 1 {
            //eprint!("Found multiple divs with title class");
        } else {
            for (_i, content) in matching_divs.iter().enumerate() {
                let content = content.inner_html();
                let content = content.split(trim_char).next().unwrap_or(&content);
                if let Ok(extracted_price) =
                    content.replace("€", "").replace(" ", "").trim().parse()
                {
                    price = extracted_price;
                }
            }
        }
        let mut surface: u16 = 0;
        let format_p = format(format_args!("span.{}", surface_class));
        let selector = Selector::parse(&format_p).unwrap();
        let matching_divs: Vec<_> = element.select(&selector).collect();
        if matching_divs.is_empty() {
            // print!("No divs found for title");
        } else if matching_divs.len() > 1 {
            //  eprint!("Found multiple divs with title class");
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
#[derive(Serialize, Deserialize)]
struct Region {
    name: String,
    cities: Vec<City>,
}

#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    apartments: Vec<AppartmentData>,
}
impl City {
    // Constructor for a new City without apartments
    pub fn new(name: String) -> Self {
        City {
            name,
            apartments: Vec::new(),
        }
    }
}
