use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;

static REGEX_CACHE: Lazy<Mutex<RegexCache>> = Lazy::new(|| Mutex::new(RegexCache::default()));

#[derive(Default)]
pub struct RegexCache {
    patterns: HashMap<String, Regex>,
}

impl RegexCache {
    fn get(&mut self, pattern: &str) -> &Regex {
        if !self.patterns.contains_key(pattern) {
            let regex = Regex::new(pattern).unwrap();
            self.patterns.insert(pattern.to_string(), regex);
        }
        self.patterns.get(pattern).unwrap()
    }

    pub fn matches(pattern: &str, text: &str) -> bool {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).is_match(text)
    }

    pub fn find<'a>(pattern: &str, text: &'a str) -> Option<regex::Match<'a>> {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).find(text)
    }

    pub fn captures<'a>(pattern: &str, text: &'a str) -> Option<regex::Captures<'a>> {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).captures(text)
    }

    pub fn replace_all<'a>(pattern: &str, text: &'a str, rep: &'a str) -> Cow<'a, str> {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).replace_all(text, rep)
    }
}
