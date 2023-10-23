#[allow(dead_code)]
enum Flight {
    Economy,
    Business,
    First,
}
#[allow(dead_code)]
enum HouseLocation {
    Number(u32),
    Street(String),
    Unknown,
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enum_flight() {
        let flight = Flight::Economy;

        match flight {
            Flight::Economy => println!("Economy"),
            Flight::Business => println!("Business"),
            Flight::First => println!("First class"),
        }
    }

    #[test]
    fn test_enum_house() {
        // let house = HouseLocation::Number(10);
        let house = HouseLocation::Street("Main Street".to_string());
        // let house = HouseLocation::Unknown;

        match house {
            HouseLocation::Number(n) => println!("House number is {}", n),
            HouseLocation::Street(s) => println!("House street is {}", s),
            HouseLocation::Unknown => println!("Unknown house"),
        }

        println!("house size is {:?}", std::mem::size_of::<HouseLocation>())
    }
}
