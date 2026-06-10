use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use std::error::Error;

pub trait ElementRefExt {
    fn get_text(&self) -> String;
    fn select_elem(&self, selector: &Selector, element: &str) -> Result<ElementRef, String>;
    fn select_text(&self, selector: &Selector, element: &str) -> Result<String, String>;
    fn select_attr(&self, attr: &str, element: &str) -> Result<String, String>;
}

impl<'a> ElementRefExt for ElementRef<'a> {
    fn get_text(&self) -> String {
        self.text()
            .flat_map(|s| s.split_whitespace())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn select_elem(&self, selector: &Selector, element: &str) -> Result<ElementRef, String> {
        self.select(selector).next().ok_or(format!("{element} not found"))
    }

    fn select_text(&self, selector: &Selector, element: &str) -> Result<String, String> {
        Ok(self.select(selector).next().ok_or(format!("{element} not found"))?.get_text())
    }

    fn select_attr(&self, attr: &str, element: &str) -> Result<String, String> {
        Ok(self.attr(attr).ok_or(format!("{element} not found"))?.to_string())
    }
}

pub fn extract_basics(
    element: ElementRef,
    title_sel: &Selector,
    image_sel: &Selector
) -> Result<(String, String, String), String> {
    let title_elem = element.select_elem(title_sel, "title")?;
    let mut url = title_elem.select_attr("href", "url")?;
    let title = title_elem.get_text();
    
    let image_elem = element.select_elem(image_sel, "image")?.value();
    let mut image_opt = None;
    for image_attr in vec!["data-full-size-image-url", "src", "data-original", "data-src", "data-lazy-src", "data-nectar-img-src"] {
        if let Some(value) = image_elem.attr(image_attr) {
            if value.starts_with("http") && !value.ends_with(".gif") {
                image_opt = Some(value.to_string());
                break;
            }
        }
    }
    let mut image = match image_opt {
        Some(img) => img,
        None => return Err("image url not found".to_string())
    };
    
    Ok((title, url, image))
}

pub fn extract_prices(
    element: ElementRef,
    price_sel: &Selector,
    old_price_sel: &Selector,
    price_sel_2: &Option<Lazy<Selector>>
) -> Result<(i32, Option<i32>), Box<dyn Error>> {
    if let Some(price_sel_2) = price_sel_2 {
        match element.select(old_price_sel).next() {
            Some(p) => {
                let price = element.select_text(price_sel_2, "price")?;
                Ok((parse_price(&price)?, Some(parse_price(&p.get_text())?)))
            },
            None => Ok((parse_price(&element.select_text(price_sel, "price")?)?, None)),
        }
    } else {
        let price = parse_price(&element.select_text(price_sel, "price")?)?;
        let old_price = element.select(old_price_sel).next().map(|p| parse_price(&p.get_text())).transpose()?;
        Ok((price, old_price))
    }
}

pub fn parse_price(text: &str) -> Result<i32, Box<dyn Error>> {
    let clean_text = text
        .replace("DT", "")
        .replace("TND", "")
        .replace("TTC", "")
        .replace(" ", "")
        .replace('\u{a0}', "");

    let price = if clean_text.contains(',') && clean_text.contains('.')
        && clean_text.find(',') < clean_text.find('.') {
        // "1,369.000"
        clean_text.replace(',', "")
            .split('.')
            .next()
            .unwrap_or(&clean_text)
            .parse::<i32>()
    } else if clean_text.contains(',') {
        // "1.369,000" or "1369,000"
        clean_text.replace('.', "")
            .split(',')
            .next()
            .unwrap_or(&clean_text)
            .parse::<i32>()
    } else {
        // "1369.000" or "1369"
        clean_text.split('.')
            .next()
            .unwrap_or(&clean_text)
            .parse::<i32>()
    };

    price.map_err(|err| format!("Failed to parse price `{text}`: {err}").into())
}

pub fn validate_url(url: &str) -> Result<(), String> {
    if url.is_empty() {
        return Err("url is empty".to_string());
    }

    //TODO: if contains base64 then fetch the product page to get image url

    // if url.contains("base64") {
    //     return Err(format!("url is encoded, probably the page isn't fully loaded: {url}"));
    // }

    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(format!("url must start with http:// or https://: `{url}`"));
    }

    if url.contains(' ') {
        return Err(format!("url contains spaces: `{url}`"));
    }

    Ok(())
}

pub fn remove_words(text: &str, words: &[&str]) -> String {
    let mut text = text.to_string();
    words.iter().for_each(|w| text = text.replace(*w, ""));
    text
}