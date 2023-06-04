pub fn get_csv_data(filename: &str) -> Vec<Vec<String>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(filename)
        .unwrap();
    let mut data2019: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let mut row: Vec<String> = Vec::new();
        for field in record.iter() {
            row.push(field.to_string());
        }
        data2019.push(row);
    }
    data2019
}
