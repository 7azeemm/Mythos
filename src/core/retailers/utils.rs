use crate::core::tracking::scrape_error::{ProductParseError, UrlError};
use crate::utils::scraper_ext::ElementRefExt;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};

static IMAGE_ATTRIBUTES: &[&str] = &["data-full-size-image-url", "data-original", "data-src", "data-lazy-src", "data-nectar-img-src", "src"];

pub fn extract_basics(element: ElementRef, title_sel: &Selector, image_sel: &Selector) -> Result<(String, String, String), ProductParseError> {
    let title_elem = element.select_elem(title_sel, "title")?;
    let url = title_elem.select_attr("href", "url")?;
    let title = title_elem.get_text();

    let image_elem = element.select_elem(image_sel, "image")?.value();
    let mut image_opt = None;
    for image_attr in IMAGE_ATTRIBUTES {
        if let Some(value) = image_elem.attr(image_attr) {
            if value.starts_with("http") {
                image_opt = Some(value.to_string());
                break;
            }
        }
    }
    
    let Some(image) = image_opt else {
        return Err(ProductParseError::MissingImageUrl)
    };

    Ok((title, url, image))
}

pub fn extract_prices(element: ElementRef, price_sel: &Selector, old_price_sel: &Selector, price_sel_2: &Option<Lazy<Selector>>) -> Result<(i32, Option<i32>), ProductParseError> {
    if let Some(price_sel_2) = price_sel_2 {
        match element.select(old_price_sel).next() {
            Some(p) => {
                let price = element.select_text(price_sel_2, "price")?;
                Ok((parse_price(&price)?, Some(parse_price(&p.get_text())?)))
            }
            None => Ok((parse_price(&element.select_text(price_sel, "price")?)?, None)),
        }
    } else {
        let price = parse_price(&element.select_text(price_sel, "price")?)?;
        let old_price = element.select(old_price_sel).next().map(|p| parse_price(&p.get_text())).transpose()?;
        Ok((price, old_price))
    }
}

pub fn parse_price(text: &str) -> Result<i32, ProductParseError> {
    let clean_text = text.replace("DT", "").replace("TND", "").replace("TTC", "").replace(" ", "").replace('\u{a0}', "");

    let price = if clean_text.contains(',') && clean_text.contains('.') && clean_text.find(',') < clean_text.find('.') {
        // "1,369.000"
        clean_text.replace(',', "").split('.').next().unwrap_or(&clean_text).parse::<i32>()
    } else if clean_text.contains(',') {
        // "1.369,000" or "1369,000"
        clean_text.replace('.', "").split(',').next().unwrap_or(&clean_text).parse::<i32>()
    } else {
        // "1369.000" or "1369"
        clean_text.split('.').next().unwrap_or(&clean_text).parse::<i32>()
    };

    price.map_err(|_| ProductParseError::InvalidPrice { value: text.to_string() })
}

pub fn validate_url(url: &str) -> Result<(), UrlError> {
    let result = if url.is_empty() {
        Err(UrlError::Empty)
    } else if !(url.starts_with("http://") || url.starts_with("https://")) {
        Err(UrlError::InvalidScheme { value: url.to_string() })
    } else if url.chars().any(char::is_whitespace) {
        Err(UrlError::ContainsSpaces { value: url.to_string() })
    } else if reqwest::Url::parse(url).ok().is_none_or(|parsed|
        parsed.host_str().is_none() || !parsed.username().is_empty() || parsed.password().is_some()
    ) {
        Err(UrlError::Malformed { value: url.to_string() })
    } else {
        Ok(())
    };
    result
}
