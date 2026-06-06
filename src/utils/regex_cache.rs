use once_cell::sync::Lazy;
use regex::{CaptureMatches, Captures, Match, Matches, Regex};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr};
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

    pub fn custom_match(pattern: &str, text: &str) -> bool {
        let mut cache = REGEX_CACHE.lock().unwrap();

        if let Some(pattern) = pattern.strip_prefix("#") {
            return cache.get(&format!("(?i){pattern}")).is_match(text)
        }

        for part in pattern.split("&") {
            let (part, negate) = match part.strip_prefix("!=") {
                Some(p) => (p, true),
                None => (part, false)
            };
            let pattern = format!("(?i)\\b(?:{part})\\b");
            let matches = cache.get(&pattern).is_match(text);
            if matches == negate {
                return false
            }
        }

        true
    }

    pub fn matches(pattern: &str, text: &str) -> bool {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).is_match(text)
    }

    pub fn captures<'a>(pattern: &str, text: &'a str) -> Option<Captures<'a>> {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).captures(text)
    }

    pub fn find_iter<'a>(pattern: &str, text: &'a str) -> Vec<Match<'a>> {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).find_iter(text).collect()
    }

    pub fn captures_iter<'a>(pattern: &str, text: &'a str) -> Vec<Captures<'a>> {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).captures_iter(text).collect()
    }

    pub fn replace_all<'a>(pattern: &str, text: &'a str, rep: &'a str) -> Cow<'a, str> {
        let mut cache = REGEX_CACHE.lock().unwrap();
        cache.get(pattern).replace_all(text, rep)
    }
}
