use std::io;

fn min_switches(start: &mut [u8], target: &[u8]) -> usize {
    let mut switch_count = 0;
    for i in 0..start.len() {
        if start[i] == target[i] {
            continue;
        } else {
            // the star state at i is different between target and start so we'll need to switch at
            // least once switch i:
            switch_count += 1;
            // if we're at the end of the slice, there are no more switch to consider.
            if i >= start.len() - 1 {
                continue;
            }
            // otherwise there can only be one previous state that allowed switching the state at
            // i:
            //       i
            //  [... x 1 0 0 0 0.....0 ]
            // we need to count how many switches to activate to reach that state
            let mut new_target = vec![0; start.len() - i - 1];
            let first = new_target.get_mut(0).unwrap();
            *first = 1;
            switch_count += min_switches(&mut start[i + 1..], &new_target);

            // now our new state is: [... x 1 0 0 0 0.....0 ], we'll keep counting from there
            start[i + 1..].copy_from_slice(&new_target);
        }
    }
    switch_count
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let start = input_line.trim_end().to_string();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let target = input_line.trim_end().to_string();

    let mut start: Vec<u8> = start
        .chars()
        .map(|s| s.to_string().parse::<u8>().unwrap())
        .collect();
    let target: Vec<u8> = target
        .chars()
        .map(|s| s.to_string().parse::<u8>().unwrap())
        .collect();

    if (start.len() != target.len()) || (start.len() > 25) {
        panic!()
    }

    println!("{}", min_switches(&mut start, &target));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_star() {
        assert_eq!(1, min_switches(&mut [0], &[1]));
        assert_eq!(1, min_switches(&mut [1], &[0]));
        assert_eq!(0, min_switches(&mut [0], &[0]));
        assert_eq!(0, min_switches(&mut [1], &[1]));
    }

    #[test]
    fn two_stars() {
        assert_eq!(0, min_switches(&mut [0, 0], &[0, 0]));
        assert_eq!(0, min_switches(&mut [1, 1], &[1, 1]));

        assert_eq!(0, min_switches(&mut [0, 1], &[0, 1]));
        assert_eq!(0, min_switches(&mut [1, 0], &[1, 0]));
        assert_eq!(0, min_switches(&mut [1, 1], &[1, 1]));

        assert_eq!(1, min_switches(&mut [1, 1], &[1, 0]));
        assert_eq!(1, min_switches(&mut [1, 0], &[1, 1]));
        assert_eq!(1, min_switches(&mut [0, 0], &[0, 1]));
        assert_eq!(1, min_switches(&mut [0, 1], &[0, 0]));

        assert_eq!(3, min_switches(&mut [0, 0], &[1, 0]));
        assert_eq!(2, min_switches(&mut [0, 0], &[1, 1]));
        assert_eq!(2, min_switches(&mut [1, 1], &[0, 0]));
        assert_eq!(3, min_switches(&mut [1, 0], &[0, 0]));
    }

    #[test]
    fn more_stars() {
        assert_eq!(2, min_switches(&mut [1, 1, 0, 1], &[0, 1, 0, 0]));

        assert_eq!(
            26,
            min_switches(&mut [1, 0, 1, 0, 1, 0], &[0, 1, 0, 1, 0, 1])
        );

        assert_eq!(
            877,
            min_switches(
                &mut [1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
                &[1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1]
            )
        );
    }
}
