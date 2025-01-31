use rand::Rng;

pub struct Validator {
    pub address: String,
    pub stake: u64,
}

pub struct PoS {
    pub validators: Vec<Validator>,
}

impl PoS {
    pub fn new() -> Self {
        PoS { validators: vec![] }
    }

    pub fn add_validator(&mut self, address: String, stake: u64) {
        self.validators.push(Validator { address, stake });
    }

    pub fn select_validator(&self) -> Option<&Validator> {
        let total_stake: u64 = self.validators.iter().map(|v| v.stake).sum();
        if total_stake == 0 { return None; }

        let mut rng = rand::rng();        ;
        let random_number: u64 = rng.gen_range(0..=total_stake);

        let mut cumulative = 0;
        for validator in &self.validators {
            cumulative += validator.stake;
            if cumulative >= random_number {
                return Some(validator);
            }
        }
        None
    }
}