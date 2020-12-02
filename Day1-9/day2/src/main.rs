type Password = (usize, usize, char, String);

fn parse_line(line: &str) -> Password {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let rng = parts[0].split("-").collect::<Vec<_>>();
    let (lo, hi) = (
        rng[0].parse::<usize>().unwrap(),
        rng[1].parse::<usize>().unwrap(),
    );
    let c = parts[1].chars().next().unwrap();

    (lo, hi, c, parts[2].to_owned())
}

fn part1(pws: &Vec<Password>) -> usize {
    pws.iter()
        .filter(|(lo, hi, c, pw)| {
            let cnt = pw.matches(c.to_string().as_str()).count();
            *lo <= cnt && cnt <= *hi
        })
        .count()
}

fn part2(pws: &Vec<Password>) -> usize {
    pws.iter()
        .filter(|(lo, hi, c, pw)| {
            let chars = pw.chars().collect::<Vec<_>>();
            (chars[*lo - 1] == *c) != (chars[*hi - 1] == *c)
        })
        .count()
}

fn main() {
    let inputs = include_str!("2.txt")
        .split_terminator("\n")
        .map(parse_line)
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
}
