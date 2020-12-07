#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

type BagReq<'a> = (&'a str, Vec<(usize, String)>);

fn parse_bag_req<'a>(req: &'a str) -> BagReq<'a> {
    lazy_static! {
        static ref BAGS: Regex = Regex::new(r"\.|bags?").unwrap();
        static ref WS: Regex = Regex::new(r"(^\s+)|(\s+$)").unwrap();
    }
    let parts = req.split(" bags contain ").collect::<Vec<_>>();
    let (bag, rest) = (parts[0], parts[1]);
    if rest.contains("no other") {
        return (bag, vec![]);
    }
    let children = BAGS
        .replace_all(rest, "")
        .split(",")
        .map(|c| {
            let w = c.trim_end().trim_start().split(" ").collect::<Vec<_>>();
            (w[0].parse::<usize>().unwrap(), w[1..].join(" "))
        })
        .collect::<Vec<_>>();

    (bag, children)
}

fn can_hold_shiny(g: &HashMap<&str, Vec<(usize, String)>>, bag: &str) -> bool {
    if bag == "shiny gold" {
        true
    } else {
        g.get(bag).unwrap().iter().any(|b| can_hold_shiny(g, &b.1))
    }
}

fn required_bags(g: &HashMap<&str, Vec<(usize, String)>>, bag: &str) -> usize {
    1 + g
        .get(bag)
        .unwrap()
        .iter()
        .map(|(cost, b)| cost * required_bags(&g, &b))
        .sum::<usize>()
}

fn main() {
    let graph = include_str!("7.txt")
        .split_terminator("\n")
        .map(|l| parse_bag_req(l))
        .collect::<HashMap<_, _>>();
    let part1 = graph.keys().filter(|b| can_hold_shiny(&graph, b)).count() - 1;
    println!("Part 1: {}", part1);

    let part2 = required_bags(&graph, "shiny gold") - 1;
    println!("Part 2: {}", part2);
}
