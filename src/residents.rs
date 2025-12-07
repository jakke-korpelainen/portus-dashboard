use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Resident {
    pub name: String,
    pub apartment: String,
    pub floor: String,
}

#[derive(Clone, Debug)]
pub struct Residents {
    pub floor: String,
    pub residents: Vec<Resident>,
}

pub struct Floor {
    pub floor: u8,
    pub resident_amount: u8,
}

impl Floor {
    const fn floor_as_roman(&self) -> &'static str {
        match self.floor {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            10 => "X",
            _ => panic!("Unsupported floor number"),
        }
    }
}

pub const APARTMENT_PREFIX: &str = "D";
pub const RESIDENTS_AMOUNT: u8 = 39;
pub const RESIDENTS_FLOORS: [Floor; 5] = [
    Floor {
        floor: 6,
        resident_amount: 8,
    },
    Floor {
        floor: 5,
        resident_amount: 8,
    },
    Floor {
        floor: 4,
        resident_amount: 8,
    },
    Floor {
        floor: 3,
        resident_amount: 8,
    },
    Floor {
        floor: 2,
        resident_amount: 7,
    },
];

pub const COMMON_NAMES: [&str; 32] = [
    "Virtanen",
    "Korhonen",
    "Mallinkainen",
    "Mäkinen",
    "Nieminen",
    "Mäkelä",
    "Hämäläinen",
    "Laine",
    "Häkkinen",
    "Koskinen",
    "Kovalainen",
    "Järvinen",
    "Lehtonen",
    "Lehtinen",
    "Saarinen",
    "Salminen",
    "Heinonen",
    "Niemi",
    "Heikkilä",
    "Kinnunen",
    "Salonen",
    "Turunen",
    "Salo",
    "Laitinen",
    "Tuominen",
    "Rantanen",
    "Karjalainen",
    "Jokinen",
    "Mattila",
    "Savolainen",
    "Lahtinen",
    "Ahonen",
];

fn get_random_name() -> String {
    let mut rng = &mut thread_rng();
    let random_name = COMMON_NAMES.choose(&mut rng).unwrap();

    if !rng.gen_bool(0.54) {
        return random_name.to_string();
    }

    let binding = COMMON_NAMES
        .iter()
        .filter(|&name| name != random_name)
        .collect::<Vec<_>>();

    let second_name = binding.choose(&mut rng).unwrap();

    format!(
        "{}{}{}",
        random_name,
        if rng.gen_bool(0.5) { "-" } else { " " },
        second_name
    )
}

pub fn get_resident_floor(apartment_number: u8) -> String {
    let mut remaining = RESIDENTS_AMOUNT;
    for floor in &RESIDENTS_FLOORS {
        remaining -= floor.resident_amount;
        if apartment_number > remaining {
            return floor.floor_as_roman().to_string();
        }
    }
    String::new()
}

pub fn get_residents() -> Vec<Residents> {
    let mut residents = Vec::with_capacity(RESIDENTS_AMOUNT as usize);

    // Create residents from highest apartment number to lowest
    for index in (1..=RESIDENTS_AMOUNT).rev() {
        residents.push(Resident {
            name: get_random_name().to_uppercase(),
            floor: get_resident_floor(index),
            apartment: format!("{}{}", APARTMENT_PREFIX, index),
        });
    }

    let mut residents_in_floors = Vec::with_capacity(RESIDENTS_FLOORS.len());

    for floor in &RESIDENTS_FLOORS {
        let floor_roman = floor.floor_as_roman().to_string();
        let floor_residents: Vec<Resident> = residents
            .iter()
            .filter(|resident| resident.floor == floor_roman)
            .cloned()
            .collect();

        residents_in_floors.push(Residents {
            floor: floor_roman,
            residents: floor_residents,
        });
    }

    residents_in_floors
}
