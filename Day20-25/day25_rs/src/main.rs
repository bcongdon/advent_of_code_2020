fn determine_encryption_key(key1: u64, key2: u64) -> u64 {
    let (mut value, mut encryption_key) = (1, 1);

    while value != key2 {
        value = (value * 7) % 20201227;
        encryption_key = (encryption_key * key1) % 20201227;
    }

    encryption_key
}

fn main() {
    let (door_pub, card_pub) = (12578151, 5051300);

    println!("Part 1: {}", determine_encryption_key(door_pub, card_pub));
}
