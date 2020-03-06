use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Flags<'a> {
    flags: HashMap<&'a str, Option<&'a str>>,
}

impl<'a> Flags<'a> {
    pub fn new(tokens: Vec<&'a str>) -> Result<Self, String> {
        match try_parse_flags(tokens) {
            Ok(flags) => Ok(Flags { flags }),
            Err(error) => Err(error),
        }
    }

    pub fn get(&'a self, flag_name: &'a str) -> Option<Option<&'a str>> {
        match self.flags.get(flag_name) {
            Some(&flag_value) => Some(flag_value),
            None => None,
        }
    }

    pub fn get_all(&'a self) -> Vec<(&'a str, Option<&'a str>)> {
        self.flags
            .iter()
            .map(|(&flag_name, &flag_value)| (flag_name, flag_value))
            .collect()
    }
}

fn try_parse_flags<'a>(tokens: Vec<&'a str>) -> Result<HashMap<&'a str, Option<&'a str>>, String> {
    let mut flags: HashMap<&'a str, Option<&'a str>> = HashMap::new();
    let mut deque: VecDeque<&'a str> = tokens.into_iter().collect();
    while deque.len() > 0 {
        if let Some(flag) = deque.pop_front() {
            if let Some(flag_name) = validate_flag(flag) {
                if let Some(next) = deque.front() {
                    if validate_flag(&next).is_some() {
                        flags.insert(flag_name, None);
                    } else {
                        let flag_value = deque.pop_front();
                        flags.insert(flag_name, flag_value);
                    }
                } else {
                    flags.insert(flag_name, None);
                }
            } else {
                return Err(format!("Invalid flag {}", flag.to_string()));
            }
        }
    }
    Ok(flags)
}

fn validate_flag(flag: &str) -> Option<&str> {
    if flag.len() == 0 {
        return None;
    }
    let (first, rest) = flag.split_at(1);
    match first {
        "-" => Some(rest),
        _ => None,
    }
}
