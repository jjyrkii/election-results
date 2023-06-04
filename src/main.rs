mod models;
mod utils;

use crate::models::res_2019;
use crate::models::res_2014;
use crate::models::region;

use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -r --region [REGION] "Region to get results for"
            )
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let mut name: String = String::from("Landesergebnis");
    if let Some(region) = matches.get_one::<String>("region") {
        name = region.to_string()
    }

    let areas = region::get_regions();
    let region = areas.iter().find(|&x| x.region_description == name).unwrap();
    let data2019 = res_2019::get_election_results("files/2019/KVGesamt.csv");
    let data2014 = res_2014::get_election_results("files/2014/KVGesamt.csv");
    let region_data_2019 = data2019.iter().find(|&x| x.region_key == region.region_key).unwrap();
    let region_data_2014 = data2014.iter().find(|&x| x.region_key == region.region_key).unwrap();

    println!("Vergleich für {}:", name);
    println!("           {0: <15} | {1: <15} | {2: <17} | {3: <15} | {4: <14} | {5: <12} | {6: <15}",
             "Wahlberechtigte",
             "Wähler",
             "Ungültige Stimmen",
             "Gültige Stimmen",
             "Partei Stimmen",
             "Sitze gesamt",
             "Sitze Partei",
    );
    println!("           {0: <15} | {1: <15} | {2: <17} | {3: <15} | {4: <14} | {5: <12} | {6: <15}",
             "Anzahl",
             "Anzahl | %",
             "Anzahl | %",
             "Anzahl | %",
             "Anzahl | %",
             "Anzahl",
             "Anzahl",
    );
    println!("2019:      {0: <15} | {1: <15} | {2: <17} | {3: <15} | {4: <14} | {5: <12} | {6: <15}",
             region_data_2019.eligible_voters,
             format!("{0: <6} | {1: <6}", region_data_2019.voters, region_data_2019.voter_turnout),
             format!("{0: <6} | {1: <6}", region_data_2019.invalid_ballots, region_data_2019.invalid_ballots_percent),
             format!("{0: <6} | {1: <6}", region_data_2019.valid_ballots, region_data_2019.valid_ballots_percent),
             format!("{0: <6} | {1: <5}", region_data_2019.votes_per_party.gruene, region_data_2019.votes_per_party_percent.gruene),
             region_data_2019.seats_overall,
             region_data_2019.seats_per_party.gruene,
    );
    println!("2014:      {0: <15} | {1: <15} | {2: <17} | {3: <15} | {4: <14} | {5: <12} | {6: <15}",
             region_data_2014.eligible_voters,
             format!("{0: <6} | {1: <6}", region_data_2014.voters, region_data_2014.voter_turnout),
             format!("{0: <6} | {1: <6}", region_data_2014.invalid_ballots, region_data_2014.invalid_ballots_percent),
             format!("{0: <6} | {1: <6}", region_data_2014.valid_ballots, region_data_2014.valid_ballots_percent),
             format!("{0: <6} | {1: <5}", region_data_2014.votes_per_party.gruene, region_data_2014.votes_per_party_percent.gruene),
             region_data_2014.seats_overall,
             region_data_2014.seats_per_party.gruene,
    );
    println!("Differenz: {0: <15} | {1: <15} | {2: <17} | {3: <15} | {4: <14} | {5: <12} | {6: <15}",
             region_data_2019.eligible_voters - region_data_2014.eligible_voters,
             format!("{0: <6} | {1: <6.1}", region_data_2019.voters - region_data_2014.voters, region_data_2019.voter_turnout - region_data_2014.voter_turnout),
             format!("{0: <6} | {1: <6.1}", region_data_2019.invalid_ballots - region_data_2014.invalid_ballots, region_data_2019.invalid_ballots_percent - region_data_2014.invalid_ballots_percent),
             format!("{0: <6} | {1: <6.1}", region_data_2019.valid_ballots - region_data_2014.valid_ballots, region_data_2019.valid_ballots_percent - region_data_2014.valid_ballots_percent),
             format!("{0: <6} | {1: <5.1}", region_data_2019.votes_per_party.gruene - region_data_2014.votes_per_party.gruene, region_data_2019.votes_per_party_percent.gruene - region_data_2014.votes_per_party_percent.gruene),
             region_data_2019.seats_overall - region_data_2014.seats_overall,
             region_data_2019.seats_per_party.gruene - region_data_2014.seats_per_party.gruene,
    );
}
