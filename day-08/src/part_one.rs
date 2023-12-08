use super::parsing::{Direction, Map};

pub fn solve(data: &str) -> u32 {
    let map = Map::parse(data);

    let mut cursor = 0;
    let mut curr = map.nodes.get("AAA").unwrap();
    loop {
        if curr.name == "ZZZ" {
            break;
        }

        let dir = &map.directions[cursor % map.directions.len()];
        let target = match dir {
            Direction::Left => &curr.left,
            Direction::Right => &curr.right,
        };

        curr = map.nodes.get(target.as_str()).unwrap();
        cursor += 1;
    }

    cursor as u32
}
