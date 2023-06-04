use crate::utils::csv::get_csv_data;
use crate::utils::parser::parse_to_int;

#[derive(Debug)]
pub struct Region {
    pub region_key: isize,
    pub region_description: String,
    pub district_count: isize,
    pub hierarchy_level: isize,
}

impl Region {
    pub fn new(
        region_key: isize,
        region_description: String,
        district_count: isize,
        hierarchy_level: isize,
    ) -> Region {
        Region {
            region_key,
            region_description,
            district_count,
            hierarchy_level,
        }
    }
}

pub fn get_regions() -> Vec<Region> {
    let new = get_csv_data("files/KVGebiet.csv");
    let mut regions: Vec<Region> = Vec::new();
    for row in new.iter() {
        let region = Region::new(
            parse_to_int(row[0].clone()),
            row[1].clone(),
            parse_to_int(row[2].clone()),
            parse_to_int(row[3].clone()),
        );
        regions.push(region);
    }
    regions
}

