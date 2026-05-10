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
        let letter = attempt[i];

        if result[i] == '🟩' {
            continue;
        }

        // Use get_mut to find the key and decrement it if we use it
        result[i] = match hash_map.get_mut(&letter) {
            None => '🟥',
            Some(count) if *count == 0 => '🟥', // If we used them all up, it's red
            Some(count) => {
                *count -= 1; // Decrement the available count!
                '🟨'
            }
        };
    }

    result
}

fn main() {
    let word = ['a', 'p', 'p', 'l', 'e'];
    let attempt = ['p', 'i', 'a', 'p', 'p'];
    println!("Result: {:?}", check(attempt, word));
}
