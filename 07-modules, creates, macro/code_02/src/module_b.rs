mod animal {
    pub struct Characteristics {
        pub height: u8,
        pub species: String,
    }

    impl Characteristics {
        pub fn create(new_species: &str) -> Self {
            Self {
                species: String::from(new_species),
                height: 30,
            }
        }
    }
}

pub fn create_animals() {
    let mut animal1 = animal::Characteristics::create("Feline");
    animal1.species = String::from("Canine");

    let animal2 = animal::Characteristics {
        height: 50,
        species: String::from("Avian"),
    };
}
