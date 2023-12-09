use super::parsing;

pub fn solve(data: &str) -> Vec<i64> {
    parsing::parse(data).iter().map(predict_next).collect()
}

fn predict_next(history: &Vec<i64>) -> i64 {
    if history.iter().all(|n| *n == 0) {
        return 0;
    }

    let mut diffs = Vec::new();
    for i in 0..history.len() - 1 {
        let diff = history[i + 1] - history[i];
        diffs.push(diff);
    }

    let next = predict_next(&diffs);
    history.last().unwrap() + next
}
