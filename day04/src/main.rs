use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::option::Option;
use std::vec::Vec;

#[derive(Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    /// Checks if passport contains all required fields.
    fn check_required_fields(&self) -> bool {
        self.birth_year != None
            && self.issue_year != None
            && self.expiration_year != None
            && self.height != None
            && self.hair_color != None
            && self.eye_color != None
            && self.passport_id != None
    }

    /// Validates all required passport fields.
    fn validate_fields(&self) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
    }

    /// Validates birth year.
    fn validate_byr(&self) -> bool {
        Passport::validate_year(&self.birth_year, 1920, 2002)
    }

    /// Validate issue year.
    fn validate_iyr(&self) -> bool {
        Passport::validate_year(&self.issue_year, 2010, 2020)
    }

    /// Validates expiration year.
    fn validate_eyr(&self) -> bool {
        Passport::validate_year(&self.expiration_year, 2020, 2030)
    }

    /// Validates height.
    fn validate_hgt(&self) -> bool {
        match &self.height {
            Some(v) => {
                let hgt_re = Regex::new(r"^(?P<value>\d+)(?P<metric>cm|in)$").unwrap();
                let hgt_slice = &v[..];
                if hgt_re.is_match(hgt_slice) {
                    let caps = hgt_re.captures(hgt_slice).unwrap();
                    let v = caps["value"].parse::<u32>().unwrap();
                    match &caps["metric"] {
                        "cm" => v >= 150 && v <= 193,
                        "in" => v >= 59 && v <= 76,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Validates hair color.
    fn validate_hcl(&self) -> bool {
        match &self.hair_color {
            Some(v) => {
                let hcl_re = Regex::new(r"^#[\d|a|b|c|d|e|f]{6}$").unwrap();
                hcl_re.is_match(&v[..])
            }
            None => false,
        }
    }

    /// Validates eye color.
    fn validate_ecl(&self) -> bool {
        match &self.eye_color {
            Some(v) => match &v[..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            None => false,
        }
    }

    /// Validates passport id.
    fn validate_pid(&self) -> bool {
        match &self.passport_id {
            Some(v) => {
                let pid_re = Regex::new(r"^\d{9}$").unwrap();
                pid_re.is_match(&v[..])
            }
            None => false,
        }
    }

    /// Validates year format and range.
    fn validate_year(year: &Option<String>, lower: u32, upper: u32) -> bool {
        match year {
            Some(v) => {
                let year_re = Regex::new(r"^\d{4}$").unwrap();
                if !year_re.is_match(&v[..]) {
                    false
                } else {
                    let v = v.parse::<u32>().unwrap();
                    v >= lower && v <= upper
                }
            }
            None => false,
        }
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut passports: Vec<Passport> = Vec::new();
    let field_re = Regex::new(r"(?P<field>\w{3}):(?P<val>#?\w+)").unwrap();

    while reader.read_line(&mut line)? > 0 {
        let mut curr_passport = Passport::new();
        loop {
            for caps in field_re.captures_iter(&line) {
                match &caps["field"] {
                    "byr" => curr_passport.birth_year = Some(caps["val"].to_string()),
                    "iyr" => curr_passport.issue_year = Some(caps["val"].to_string()),
                    "eyr" => curr_passport.expiration_year = Some(caps["val"].to_string()),
                    "hgt" => curr_passport.height = Some(caps["val"].to_string()),
                    "hcl" => curr_passport.hair_color = Some(caps["val"].to_string()),
                    "ecl" => curr_passport.eye_color = Some(caps["val"].to_string()),
                    "pid" => curr_passport.passport_id = Some(caps["val"].to_string()),
                    "cid" => curr_passport.country_id = Some(caps["val"].to_string()),
                    _ => panic!("Attribute not found."),
                }
            }
            line.clear();
            if reader.read_line(&mut line)? == 0 || line.trim().len() == 0 {
                break;
            }
        }
        passports.push(curr_passport);
        line.clear();
    }

    let mut n_filled = 0;
    let mut n_valid = 0;
    for passport in passports {
        if passport.check_required_fields() {
            n_filled += 1
        }
        if passport.validate_fields() {
            n_valid += 1
        }
    }
    println!("{}", n_filled);
    println!("{}", n_valid);

    Ok(())
}
