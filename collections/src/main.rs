use std::collections::HashMap;

fn main() {
    println!("Analysing vector!");

    let mut my_vec = vec![10, 20, 30, 30, 15, 45];
    println!("{:?}", analyse_vector(&mut my_vec[..]));

    let start_text = String::from("hello world and welcome to my awesome program");
    println!("Translating '{}' to pig latin", start_text);
    println!("{}", to_pig_latin(&start_text));
}

#[derive(Debug)]
struct VectorStats {
    mean: u32,
    median: u32,
    mode: u32,
}

fn analyse_vector(input: &mut [u32]) -> VectorStats {
    let mut sum = 0;
    let mut occurrences = HashMap::new();

    for &mut value in input.into_iter() {
        let count = occurrences.entry(value).or_insert(0);
        *count += 1;

        sum += value;
    }

    input.sort_unstable();
    let mode = *occurrences.iter().max_by_key(|x| x.1).unwrap().0;

    return VectorStats {
        mean: (sum / input.len() as u32),
        median: *input.get(input.len() / 2 as usize).unwrap(),
        mode,
    };
}

fn to_pig_latin(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for word in input.split_whitespace() {
        let mut test = word_to_pig_latin(word);
        test.push(' ');

        result += &test[..];
    }

    return result;
}

fn word_to_pig_latin(input: &str) -> String {
    let first_char = input.chars().next().unwrap();
    let is_vowel = ['a', 'e', 'i', 'o', 'u'].contains(&first_char.to_lowercase().next().unwrap());

    if is_vowel {
        return input.to_owned() + "hay";
    } else {
        let mut chars = input.chars();
        chars.next();

        return format!("{}{}ay", chars.as_str(), first_char);
    }
}
