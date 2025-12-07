use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Resident {
    pub name: String,
    pub apartment: String,
    pub floor: u8,
}

#[derive(Clone, Debug)]
pub struct Residents {
    pub floor: u8,
    pub residents: Vec<Resident>,
}

pub struct Floor {
    pub floor: u8,
    pub resident_amount: u8,
}

pub const APARTMENT_PREFIX: &str = "D";
pub const RESIDENTS_AMOUNT: u8 = 39;
pub const RESIDENTS_FLOORS: [Floor; 5] = [
    Floor {
        floor: 2,
        resident_amount: 7,
    },
    Floor {
        floor: 3,
        resident_amount: 8,
    },
    Floor {
        floor: 4,
        resident_amount: 8,
    },
    Floor {
        floor: 5,
        resident_amount: 8,
    },
    Floor {
        floor: 6,
        resident_amount: 8,
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
    let mut rng = thread_rng();
    let random_name = COMMON_NAMES.choose(&mut rng).unwrap();
    if rng.gen_bool(0.54) {
        let second_name = loop {
            let name = COMMON_NAMES.choose(&mut rng).unwrap();
            if name != random_name {
                break name;
            }
        };
        if rng.gen_bool(0.5) {
            format!("{}-{}", random_name, second_name)
        } else {
            format!("{} {}", random_name, second_name)
        }
    } else {
        random_name.to_string()
    }
}

pub fn get_resident_floor(index: u8) -> u8 {
    let mut total_residents = 0;
    for floor in RESIDENTS_FLOORS.iter() {
        total_residents += floor.resident_amount;
        if index <= total_residents {
            return floor.floor;
        }
    }
    0 // Default case, should not happen if index is within bounds
}

pub fn get_residents() -> Vec<Residents> {
    let residents = (1..=RESIDENTS_AMOUNT)
        .map(|index| Resident {
            name: get_random_name().to_uppercase(),
            floor: get_resident_floor(index),
            apartment: format!("{}{}", APARTMENT_PREFIX, index),
        })
        .collect::<Vec<Resident>>();

    let mut residents_in_floors: Vec<Residents> = Vec::new();

    for floor in RESIDENTS_FLOORS.iter() {
        let floor_residents: Vec<Resident> = residents
            .iter()
            .filter(|resident| resident.floor == floor.floor)
            .cloned()
            .collect();

        residents_in_floors.push(Residents {
            floor: floor.floor,
            residents: floor_residents,
        });
    }

    residents_in_floors.clone()
}
