#[derive(Debug)]
pub struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Cubes {
    pub fn zero() -> Cubes {
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn parse(data: &str) -> Cubes {
        let mut result = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in data.split(", ") {
            let parts = cube.split(" ").collect::<Vec<_>>();
            let count = parts[0].parse::<u32>().unwrap();
            let color = parts[1];

            match color {
                "red" => result.red += count,
                "green" => result.green += count,
                "blue" => result.blue += count,
                _ => {}
            }
        }

        result
    }

    pub fn fits_within(&self, rules: &Cubes) -> bool {
        self.red <= rules.red && self.green <= rules.green && self.blue <= rules.blue
    }

    pub fn max(&self, other: &Cubes) -> Cubes {
        Cubes {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub peeks: Vec<Cubes>,
}

impl Game {
    pub fn parse(data: &str) -> Game {
        let parts = data.split(": ").collect::<Vec<_>>();
        let id = parts[0].replace("Game ", "").parse::<u32>().unwrap();
        let peeks = parts[1].split("; ").map(Cubes::parse).collect::<Vec<_>>();
        Game { id, peeks }
    }
}
