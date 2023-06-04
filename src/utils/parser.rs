pub fn parse_to_int(s: String) -> isize {
    if s == "" {
        return 0;
    }
    s.parse::<isize>().unwrap()
}

pub fn parse_to_float(s: String) -> f32 {
    if s == "" {
        return 0.0;
    }
    s.replace(",", ".").parse::<f32>().unwrap()
}

