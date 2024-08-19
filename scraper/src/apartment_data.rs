use serde::{Serialize,Deserialize};
use scraper::{Selector,ElementRef};
use std::fmt::format;
use regex::Regex;

#[derive(Debug, Serialize, PartialEq, Deserialize)]
pub struct ApartmentData {
    pub title: String,
    pub surface: u16,
    pub price: u16,
}
impl ApartmentData {
   pub fn from_div(element: &ElementRef) -> Self {
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
