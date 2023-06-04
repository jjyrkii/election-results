#[derive(Debug)]
pub struct PartyResultInt {
    pub spd: isize,
    pub cdu: isize,
    pub afd: isize,
    pub fdp: isize,
    pub gruene: isize,
    pub linke: isize,
    pub wg1: isize,
    pub wg2: isize,
    pub wg3: isize,
    pub wg4: isize,
    pub wg_sum: isize,
}

impl PartyResultInt {
    pub fn new(
        spd: isize,
        cdu: isize,
        afd: isize,
        fdp: isize,
        gruene: isize,
        linke: isize,
        wg1: isize,
        wg2: isize,
        wg3: isize,
        wg4: isize,
        wg_sum: isize,
    ) -> PartyResultInt {
        PartyResultInt {
            spd,
            cdu,
            afd,
            fdp,
            gruene,
            linke,
            wg1,
            wg2,
            wg3,
            wg4,
            wg_sum,
        }
    }
}

#[derive(Debug)]
pub struct PartyResultFloat {
    pub spd: f64,
    pub cdu: f64,
    pub afd: f64,
    pub fdp: f64,
    pub gruene: f64,
    pub linke: f64,
    pub wg1: f64,
    pub wg2: f64,
    pub wg3: f64,
    pub wg4: f64,
    pub wg_sum: f64,
}

impl PartyResultFloat {
    pub fn new(
        spd: f64,
        cdu: f64,
        afd: f64,
        fdp: f64,
        gruene: f64,
        linke: f64,
        wg1: f64,
        wg2: f64,
        wg3: f64,
        wg4: f64,
        wg_sum: f64,
    ) -> PartyResultFloat {
        PartyResultFloat {
            spd,
            cdu,
            afd,
            fdp,
            gruene,
            linke,
            wg1,
            wg2,
            wg3,
            wg4,
            wg_sum,
        }
    }
}