use futures::future::{join_all};
use regex::Regex;
use reqwest;
use scraper::{Html,Selector,ElementRef};
use std::fmt::format;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
mod apartment_data;
mod region;
mod city;
use crate::apartment_data::ApartmentData;
use crate::region::Region;
use crate::city::City;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let regions = get_regions().await?;
    for region in regions{
      write_region_to_file(region).await?;
    }
    Ok(())
}
async fn write_region_to_file(region:Region)->Result<(),Box<dyn std::error::Error>>{
    let directory = format!("./{}/",region.name);
    fs::create_dir_all(&directory)?;
    let futures: Vec<_> = region
        .cities
        .iter()
        .map(|city|{
            let path = format!("{}{}",directory,city.name);
            write_city_to_file(path,&city.name)
        })
        .collect();
    let cities = join_all(futures).await;
    for result in cities {
        if let Err(e) = result {
            eprintln!("Error writing city to file: {}", e);
        }
    }
    Ok(())
}
async fn write_city_to_file(file_path: String,city_name:&String) -> Result<(), Box<dyn std::error::Error>> {
    let mut city: City = City::new(city_name);
    scrape_city(&mut city).await?;
    if city.apartments.is_empty() {
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&city)?;
    let mut file = File::create(format!("{}.json", &file_path))?;
    println!("Writing to file: {}",file_path);
    file.write_all(json.as_bytes())?;
    Ok(())
}
async fn get_city_from_file(){
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
    const URL:&str = "https://www.olx.ro/sitemap/regions/";
    let mut regions: Vec<Region> = Vec::new();
    let client = reqwest::Client::new();

    // Send a GET request and wait for the response
    let response = client.get(URL).send().await?;

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
            for (_i, content) in matching_ul.iter().enumerate() {
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
                                        if let Some(href) = a.value().attr("href"){
                                            cities.push(City::new(&String::from(href.replace("/",""))));
                                        }
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
    let mut vec: Vec<ApartmentData> = Vec::new();
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
    let mut parsed_elements:u16=0;
    get_data(&url, &mut vec,&mut parsed_elements,results_number).await?;
    let mut counter = 2;
    let page_number = get_page_number(&url).await?;
    let mut incremented_url = String::from(&url);
    while counter <= page_number &&  
        get_data(&incremented_url, &mut vec,&mut parsed_elements,results_number).await?{
        //get_data(&incremented_url, &mut vec,&mut parsed_elements,results_number).await?;
        incremented_url = format!("{}&page={}", &url, counter);
        counter += 1;
    }

    //println!("Total number: {} for {}", &vec.len(), name);
    city.apartments = vec;
    //let (av1, av2, ratio) = city.calculate_averages();
    //println!("Average of field1: {:.2}", av1);
    //println!("Average of field2: {:.2}", av2);
    //println!("Ratio of averages (field1 / field2): {:.2}", ratio);

    Ok(())
}

async fn get_results_number(url: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    if response.status().is_success() {
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

async fn get_page_number(url: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let target_class = "css-1mi714g";
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let format = format(format_args!("a.{}", target_class));
        let selector = Selector::parse(&format).unwrap();
        let matching_ul: Vec<_> = document.select(&selector).collect();
        if matching_ul.is_empty() {
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
        return Ok(0);
    }
}

async fn get_data(
    url: &str,
    vec: &mut Vec<ApartmentData>,
    parsed_elements:&mut u16,
    results_number:u16
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let target_class = "css-1venxj6";
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let format = format(format_args!("div.{}", target_class));
        let selector = Selector::parse(&format).unwrap();
        let matching_divs: Vec<_> = document.select(&selector).collect();
        if matching_divs.is_empty() {
            return Ok(false);
        } else {
            for (_index, content) in matching_divs.iter().enumerate() {
                let ap_data = ApartmentData::from_div(content);
                match vec.last() {
                    Some(last) => {
                        if ap_data == *last {
                            return Ok(false);
                        }
                    }
                    _ => (),
                }
                if *parsed_elements >= results_number{
                    return Ok(false);
                }
                *parsed_elements+=1;
                if ap_data.surface > 15 && ap_data.price > 100 {
                    vec.push(ap_data);
                }
            }
        }
    } else {
        return Ok(false);
    }
    Ok(true)
}

