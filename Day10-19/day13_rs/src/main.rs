use itertools::Itertools;

fn main() {
    let mut lines = include_str!("13.txt").split_terminator("\n");
    let p1_timestamp = lines.next().map(|l| l.parse::<u64>().unwrap()).unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, bus)| bus.parse::<u64>().ok().map(|b| (b, idx)))
        .collect::<Vec<_>>();

    let part1 = busses
        .iter()
        .map(|(b, _)| (b - (p1_timestamp % b), b))
        .sorted()
        .next()
        .map(|(b, ts)| b * ts)
        .unwrap();
    println!("Part 1: {}", part1);

    let (mut start_ts, mut ts, mut jump) = (0, 0, 1);
    for (bus, offset) in busses {
        ts = start_ts;
        loop {
            if (ts + offset as u64) % bus == 0 {
                start_ts = ts;
                jump *= bus;
                break;
            }
            ts += jump;
        }
    }

    println!("Part 2: {}", ts);
}
