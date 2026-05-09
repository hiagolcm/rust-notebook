use std::collections::HashMap;

fn check(attempt: [char; 5], word: [char; 5]) -> [char; 5] {
    let mut hash_map: HashMap<char, u8> = HashMap::new();

    for i in 0..5 {
        hash_map
            .entry(word[i])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut result: [char; 5] = ['🟥'; 5];

    for i in 0..5 {
        if attempt[i] == word[i] {
            result[i] = '🟩';
            hash_map.entry(attempt[i]).and_modify(|count| *count -= 1);
        }
    }

    for i in 0..5 {
        let  letter = attempt[i];

        if result[i] ==  '🟩' { continue }

        result[i] = match hash_map.get(&letter) {
                None => '🟥',
                Some(&0) => '🟥',
                _ => '🟨'
            };
    }

    result
}

fn main() {
    let word = ['h', 'i', 'a', 'g', 'r'];
    let attempt = ['h', 'i', 'a', 'i', 'o'];
    println!("Result: {:?}", check(attempt, word));
}
