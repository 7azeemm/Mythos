use scraper::{ElementRef, Selector};

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