#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    pub maps: Vec<Map>,
}

#[derive(Debug)]
pub struct Map {
    pub from: String,
    pub to: String,
    pub transforms: Vec<Transform>,
}

#[derive(Debug)]
pub struct Transform {
    pub dst: u32,
    pub src: u32,
    pub len: u32,
}

impl Almanac {
    pub fn parse(content: &str) -> Almanac {
        let mut lines = content.lines().map(|l| l.trim()).peekable();

        let seeds = Self::parse_seeds(&mut lines);
        assert!(lines.next().unwrap().is_empty());

        let mut maps = Vec::new();
        while lines.peek().is_some() {
            maps.push(Map::parse(&mut lines));
        }

        Almanac { seeds, maps }
    }

    fn parse_seeds<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<u32> {
        lines
            .next()
            .unwrap()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    pub fn map(&self, system: &str, location: u32) -> (&str, u32) {
        let map = self.maps.iter().find(|m| m.from == system).unwrap();
        let new = map.map(location);
        (map.to.as_str(), new)
    }
}

impl Map {
    pub fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Map {
        let header = lines.next().unwrap().replace(" map:", "");
        let parts: Vec<_> = header.split("-").collect();

        let from = parts[0].into();
        let to = parts[2].into();

        let mut transforms = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            transforms.push(Transform::parse(line));
        }

        Map {
            from,
            to,
            transforms,
        }
    }

    pub fn map(&self, location: u32) -> u32 {
        for t in &self.transforms {
            let new = t.map(location);
            if new != location {
                return new;
            }
        }

        location
    }
}

impl Transform {
    pub fn parse(line: &str) -> Transform {
        let parts: Vec<_> = line.split(" ").filter_map(|s| s.parse().ok()).collect();

        let dst = parts[0];
        let src = parts[1];
        let len = parts[2];
        Transform { dst, src, len }
    }

    pub fn map(&self, location: u32) -> u32 {
        let offset = location as i64 - self.src as i64;
        if offset >= 0 && offset < self.len as i64 {
            self.dst + offset as u32
        } else {
            location
        }
    }
}
