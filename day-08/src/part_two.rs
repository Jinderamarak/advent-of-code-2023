use super::parsing::{Direction, Map, Node};

pub fn solve(data: &str) -> u64 {
    let map = Map::parse(data);

    let distances = map
        .nodes
        .values()
        .filter(|n| n.name.ends_with("A"))
        .map(|n| reach_z(n, &map))
        .collect::<Vec<_>>();

    lcm(&distances)
}

fn reach_z(node: &Node, map: &Map) -> u32 {
    let mut cursor = 0;
    let mut curr = node;
    loop {
        if curr.name.ends_with("Z") {
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

fn lcm(values: &[u32]) -> u64 {
    values
        .iter()
        .map(|i| *i as u64)
        .reduce(lcm_two)
        .unwrap()
}

fn lcm_two(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm_four() {
        let n = [2, 3, 4, 5];
        let lcm = lcm(&n);

        assert_eq!(60, lcm);
    }

    #[test]
    fn test_lcm_many() {
        let n = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15];
        let lcm = lcm(&n);

        assert_eq!(360360, lcm)
    }
}