use crate::models::party;
use crate::utils::csv::get_csv_data;
use crate::utils::parser::{parse_to_int, parse_to_float};

#[derive(Debug)]
pub struct ElectionResult {
    pub vote_mark: String,
    pub region_key: isize,
    pub district_mark: String,
    pub eligible_voters: isize,
    pub voters: isize,
    pub invalid_ballots: isize,
    pub valid_ballots: isize,
    pub seats_overall: isize,
    pub votes_per_party: party::PartyResultInt,
    pub seats_per_party: party::PartyResultInt,
    pub voter_turnout: f64,
    pub invalid_ballots_percent: f64,
    pub valid_ballots_percent: f64,
    pub votes_per_party_percent: party::PartyResultFloat,
    pub last_updated: String,
}

impl ElectionResult {
    pub fn new(
        vote_mark: String,
        region_key: isize,
        district_mark: String,
        eligible_voters: isize,
        voters: isize,
        invalid_ballots: isize,
        valid_ballots: isize,
        seats_overall: isize,
        votes_per_party: party::PartyResultInt,
        seats_per_party: party::PartyResultInt,
        voter_turnout: f64,
        invalid_ballots_percent: f64,
        valid_ballots_percent: f64,
        seats_per_party_percent: party::PartyResultFloat,
        last_updated: String,
    ) -> ElectionResult {
        ElectionResult {
            vote_mark,
            region_key,
            district_mark,
            eligible_voters,
            voters,
            invalid_ballots,
            valid_ballots,
            seats_overall,
            votes_per_party,
            seats_per_party,
            voter_turnout,
            invalid_ballots_percent,
            valid_ballots_percent,
            votes_per_party_percent: seats_per_party_percent,
            last_updated,
        }
    }
}

pub fn get_election_results(filename: &str) -> Vec<ElectionResult> {
    let new = get_csv_data(filename);
    let mut election_results: Vec<ElectionResult> = Vec::new();
    for row in new.iter() {
        let tmp = ElectionResult::new(
            row[0].clone(),
            parse_to_int(row[1].clone()),
            row[2].clone(),
            parse_to_int(row[3].clone()),
            parse_to_int(row[4].clone()),
            parse_to_int(row[5].clone()),
            parse_to_int(row[6].clone()),
            parse_to_int(row[7].clone()),
            party::PartyResultInt::new(
                parse_to_int(row[8].clone()),
                parse_to_int(row[9].clone()),
                parse_to_int(row[10].clone()),
                parse_to_int(row[11].clone()),
                parse_to_int(row[12].clone()),
                parse_to_int(row[13].clone()),
                parse_to_int(row[14].clone()),
                parse_to_int(row[15].clone()),
                parse_to_int(row[16].clone()),
                parse_to_int(row[17].clone()),
                parse_to_int(row[18].clone()),
            ),
            party::PartyResultInt::new(
                parse_to_int(row[28].clone()),
                parse_to_int(row[29].clone()),
                parse_to_int(row[30].clone()),
                parse_to_int(row[31].clone()),
                parse_to_int(row[32].clone()),
                parse_to_int(row[33].clone()),
                parse_to_int(row[34].clone()),
                parse_to_int(row[35].clone()),
                parse_to_int(row[36].clone()),
                parse_to_int(row[37].clone()),
                parse_to_int(row[38].clone()),
            ),
            parse_to_float(row[48].clone()),
            parse_to_float(row[49].clone()),
            parse_to_float(row[50].clone()),
            party::PartyResultFloat::new(
                parse_to_float(row[51].clone()),
                parse_to_float(row[52].clone()),
                parse_to_float(row[53].clone()),
                parse_to_float(row[54].clone()),
                parse_to_float(row[55].clone()),
                parse_to_float(row[56].clone()),
                parse_to_float(row[57].clone()),
                parse_to_float(row[58].clone()),
                parse_to_float(row[59].clone()),
                parse_to_float(row[60].clone()),
                parse_to_float(row[61].clone()),
            ),
            row[71].clone(),
        );
        election_results.push(tmp);
    }
    election_results
}