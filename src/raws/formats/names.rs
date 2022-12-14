use bracket_random::prelude::RandomNumberGenerator;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Clone, Debug)]
pub struct Names {
    pub male_first: Vec<String>,
    pub female_first: Vec<String>,
    pub last_names: Vec<String>,
}

impl Names {
    pub fn new() -> Self {
        Self { male_first: Vec::new(), female_first: Vec::new(), last_names: Vec::new() }
    }

    pub fn random_settler_name(&self, rng: &mut RandomNumberGenerator, male: bool) -> String {
        use inflector::Inflector;
        let first_source = match male {
            true => &self.male_first,
            false => &self.female_first,
        };
        let first_name = rng.random_slice_entry(first_source).unwrap().to_title_case();
        let last_name = rng.random_slice_entry(&self.last_names).unwrap().to_title_case();

        format!("{} {}", first_name, last_name)
    }
}

fn line_by_line(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(filename).expect("Unable to read names file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

pub fn load_names() -> Names {
    let mut names = Names::new();
    names.female_first = line_by_line("raws/first_names_female.txt");
    names.male_first = line_by_line("raws/first_names_male.txt");
    names.last_names = line_by_line("raws/last_names.txt");
    names
}
