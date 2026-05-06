use std::error::Error;
use std::num::ParseIntError;
use scraper::ElementRef;

pub trait ElementRefExt {
    fn get_text(&self) -> String;
}

impl<'a> ElementRefExt for ElementRef<'a> {
    fn get_text(&self) -> String {
        self.text()
            .flat_map(|s| s.split_whitespace())
            .collect::<Vec<_>>()
            .join(" ")
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

pub fn parse_url(site: &str, url: &str) -> String {
    // 1. Clean any trailing slashes (both / and \ just in case)
    let cleaned_url = url.trim_end_matches(|c| c == '/' || c == '\\');

    // 2. Get the last segment after the final slash
    let mut last_part = cleaned_url.rsplit('/').next().unwrap_or(cleaned_url);

    // 3. Safely remove ".html" from the end if it exists
    if let Some(stripped) = last_part.strip_suffix(".html") {
        last_part = stripped;
    }

    // 4. Format it using the site's name
    format!("{}/{}", site, last_part)
} 