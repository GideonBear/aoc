use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk: Vec<Option<u64>> = vec![];
    for (i, mut chunk) in input
        .to_string()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let file = chunk.next().unwrap();
        for _ in 0..file {
            disk.push(Some(i.try_into().unwrap()));
        }

        let free = chunk.next();
        if let Some(free) = free {
            for _ in 0..free {
                disk.push(None);
            }
        }
        assert!(chunk.next().is_none());
    }

    loop {
        let space = disk.iter().enumerate().find(|(_i, x)| x.is_none());
        let space = match space {
            None => break,
            Some(x) => (x.0, *x.1),
        };

        let value = disk.iter().enumerate().rfind(|(_i, x)| x.is_some());
        let value = match value {
            None => break,
            Some(x) => (x.0, *x.1),
        };

        if value.0 < space.0 {
            break;
        }

        disk[space.0] = value.1;
        disk[value.0] = None;
    }

    Some(
        disk.into_iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|x| i as u64 * x))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk: Vec<Option<u64>> = vec![];
    for (i, mut chunk) in input
        .to_string()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let file = chunk.next().unwrap();
        for _ in 0..file {
            disk.push(Some(i.try_into().unwrap()));
        }

        let free = chunk.next();
        if let Some(free) = free {
            for _ in 0..free {
                disk.push(None);
            }
        }
        assert!(chunk.next().is_none());
    }

    let highest = disk.iter().rfind(|x| x.is_some()).unwrap().unwrap();

    let mut tried = vec![];
    loop {
        // print_disk(&disk);
        println!("{} / {}", tried.len(), highest);

        let value = disk
            .iter()
            .enumerate()
            .rfind(|(_i, x)| x.is_some() && !tried.contains(&x.unwrap()));
        let value = match value {
            None => break,
            Some(x) => *x.1,
        };
        let file: Vec<_> = disk
            .iter()
            .copied()
            .enumerate()
            .filter(|(_i, x)| *x == value)
            .collect();

        let space = disk
            .iter()
            .copied()
            .enumerate()
            .chunk_by(|(_i, x)| x.is_none())
            .into_iter()
            .filter_map(|(f, chunk)| match f {
                true => Some(chunk.collect::<Vec<_>>()),
                false => None,
            })
            .find(|chunk| chunk.len() >= file.len());
        let space: Vec<(usize, Option<u64>)> = match space {
            None => {
                tried.push(value.unwrap());
                continue;
            }
            Some(x) => x.into_iter().take(file.len()).collect(),
        };

        if file.first().unwrap().0 < space.first().unwrap().0 {
            tried.push(value.unwrap());
            continue;
        }

        assert_eq!(file.len(), space.len());
        for (file_i, space_i) in file.into_iter().zip(space) {
            disk[space_i.0] = file_i.1;
            disk[file_i.0] = None;
        }
    }

    Some(
        disk.into_iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|x| i as u64 * x))
            .sum(),
    )
}

fn print_disk(disk: &Vec<Option<u64>>) {
    for part in disk {
        print!(
            "{}",
            match part {
                None => ".".to_string(),
                Some(x) => x.to_string(),
            }
        );
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
