pub fn read_file_to_string(path: &str) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(&path).expect("Cannot open file");
    let metadata = std::fs::metadata(&path).expect("Cannot read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");
    String::from_utf8(buffer).expect("Invalid UTF-8")
}

use std::collections::HashSet;
pub fn take_random_element_from_set<T: Copy + Eq + std::hash::Hash>(set: &mut HashSet<T>) -> T {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let values = set.iter().map(|i| *i).collect::<Vec<T>>();
    let chosen = values.choose(&mut rng).unwrap();
    set.remove(chosen);
    *chosen
}

use std::collections::HashMap;
pub fn print_map<K: std::fmt::Display, V: std::fmt::Display>(map: &HashMap<K, V>) {
    for (key, val) in map.iter() {
        println!("{}: {}", key, val);
    }
}

use crate::encoding::{Char, Alphabet};
pub fn index_of_coincidence(chars: &[Char], alphabet: &Alphabet) -> f64 {
    let mut counts = vec![0u64; alphabet.len()];
    for i in 0..chars.len() {
        counts[chars[i] as usize] += 1;
    }
    let mut numer = 0;
    let mut total = 0;
    for i in 0..alphabet.len() {
        numer += counts[i] * counts[i].saturating_sub(1);
        total += counts[i];
    }
    return (alphabet.len() as f64 * numer as f64) / (total as f64 * (total - 1) as f64);
}

pub const IOC_THRESHOLD: f64 = 1.55;

pub fn estimate_key_period_by_index_of_coincidence(
    chars: &[Char],
    alphabet: &Alphabet,
    ioc_threshold: f64,
) -> u32 {
    let mut period = 0u32;
    let mut slice = vec![0; chars.len()];
    loop {
        period += 1;
        let mut ioc = 0.0;
        for i in 0..period {
            let slice_len = chars.len() as u32 / period;
            slice.resize(slice_len as usize, 0);
            for j in 0..slice_len {
                slice[j as usize] = chars[(period * j + i) as usize];
            }
            ioc += index_of_coincidence(&slice, alphabet);
        }
        if ioc / period as f64 > ioc_threshold {
            break;
        }
    }
    period
}

pub fn uniform_random() -> f64 {
    use rand::distributions::{Distribution, Uniform};
    let distribution = Uniform::from(0.0..=1.0f64);
    let mut rng = rand::thread_rng();
    distribution.sample(&mut rng)
}

#[inline(always)]
pub fn probability(p: f64) -> bool {
    p > uniform_random()
}
