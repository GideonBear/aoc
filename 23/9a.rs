use std::fs;

fn predict(history: Vec<i32>) -> i32{
    let mut diffs: Vec<Vec<i32>> = vec![history];
    loop {
        let diff = get_diff(&diffs[diffs.len() - 1]);
        if diff.iter().all(|x| x == &0) {
            let mut curr = 0;
            for diff in diffs.into_iter().rev() {
                curr = diff[diff.len() - 1] + curr;
            }
            return curr;
        }
        diffs.push(diff);
    }
}

fn get_diff(prev: &Vec<i32>) -> Vec<i32> {
    let mut it = prev.iter();
    let mut curr = it.next().unwrap();
    it.map(|x| {
        let diff = x - curr;
        curr = x;
        diff
    }).collect()
}

fn main() {
    let text = fs::read_to_string("9.txt").expect("Error while reading file");

    println!(
        "{}",
        text.split('\n')
            .map(|x| x.split(' ').map(|y| y.parse().unwrap()).collect())
            .map(predict)
            .sum::<i32>()
    );
}
