#[macro_use]
extern crate lazy_static;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn parse_list(raw: &str) -> (Vec<String>, Vec<String>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*) \(contains (.*)\)").unwrap();
    }
    let m = RE.captures(raw).unwrap();
    (
        m[1].split(' ').map(|s| s.to_string()).collect(),
        m[2].split(", ").map(|s| s.to_string()).collect(),
    )
}

fn main() {
    let ing_lists = include_str!("21.txt")
        .lines()
        .map(parse_list)
        .collect::<Vec<_>>();
    let all_foods = ing_lists
        .iter()
        .flat_map(|(fs, _)| fs.iter().cloned())
        .collect::<HashSet<_>>();

    let mut allergens = ing_lists
        .iter()
        .flat_map(|(_, als)| als)
        .map(|a| (a, all_foods.clone()))
        .collect::<HashMap<_, _>>();

    for (fs, alls) in ing_lists.iter() {
        let fs: HashSet<String> = HashSet::from_iter(fs.iter().cloned());
        for a in alls {
            let set = allergens.get_mut(a).unwrap();
            *set = set.intersection(&fs).cloned().collect();
        }
    }

    let mut non_allergen_foods = HashSet::from(all_foods);
    allergens.values().flat_map(|v| v).for_each(|a| {
        non_allergen_foods.remove(a);
    });
    let part1 = ing_lists
        .iter()
        .flat_map(|(fs, _)| fs)
        .filter(|&f| non_allergen_foods.contains(f))
        .count();
    println!("Part 1: {}", part1);

    let mut allergen_map: HashMap<String, String> = HashMap::new();
    while !allergens.is_empty() {
        let (&a, fs) = allergens
            .iter()
            .filter(|(_, fs)| fs.len() == 1)
            .next()
            .unwrap();
        let f = fs.iter().next().unwrap().to_string();
        allergen_map.insert(a.to_string(), f.clone());
        for (_, suspects) in allergens.iter_mut() {
            suspects.remove(&f);
        }
        allergens.remove(a);
    }

    let part2 = allergen_map.iter().sorted().map(|(_, ing)| ing).join(",");
    println!("Part 2: {}", part2);
}
