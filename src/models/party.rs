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
    pub spd: f32,
    pub cdu: f32,
    pub afd: f32,
    pub fdp: f32,
    pub gruene: f32,
    pub linke: f32,
    pub wg1: f32,
    pub wg2: f32,
    pub wg3: f32,
    pub wg4: f32,
    pub wg_sum: f32,
}

impl PartyResultFloat {
    pub fn new(
        spd: f32,
        cdu: f32,
        afd: f32,
        fdp: f32,
        gruene: f32,
        linke: f32,
        wg1: f32,
        wg2: f32,
        wg3: f32,
        wg4: f32,
        wg_sum: f32,
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

