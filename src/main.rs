pub mod pre;
pub mod util;

pub mod encoding;
pub mod hill;
pub mod key;
pub mod stats;

use std::collections::HashMap;
pub fn print_map<K: std::fmt::Display, V: std::fmt::Display>(map: &HashMap<K, V>) {
    for (key, val) in map.iter() {
        println!("{}: {}", key, val);
    }
}

use encoding::{Char, Encoding};
pub fn read_encoded_text_from_file(
    path: &str,
    encoding: &Encoding,
    normalize: HashMap<char, char>,
) -> Vec<Char> {
    use pre::Pre;
    let pre = Pre::from_emitter_and_normalizer(encoding.char_set(), normalize);
    encoding.encode_str(&pre.process(&util::read_file_to_string(path)))
}

pub fn print_if(print: bool, message: &str) -> bool {
    if print {
        println!("{}", message);
    }
    print
}

pub fn lavy_accept(prev: f64, next: f64) -> bool {
    if next > prev {
        return true;
    }
    let degradation = next - prev;
    let p = (-degradation / TEMPERATURE).exp() - 1.0;
    p > THRESHOLD && util::probability(p)
}

pub const LATIN: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ZODIAC: &'static str = "!#%()+/=@\\^_56789ABcdDeEfFGHIjJkKlLMNOpPqQrRStTUVWXYzZ";
pub const SYMBOL: &'static str = "!@6/_^&*()%+=-|987#5$,.0[]?12";

pub const TEMPERATURE: f64 = 3500.0;
pub const THRESHOLD: f64 = 0.0085;

pub const ENERGY_VALUE: f64 = 5000.0;
pub const EXPECT_VALUE: f64 = 5.0;

fn main() {
    use encoding::Frequency;
    use hill::{Climber, Config};
    use key::Key;

    let mut pre = pre::Pre::new();
    pre::latin(&mut pre);

    let homophone_ratio = 0.2f64;

    println!("Creating alphabet and encoding");
    let output_encoding = Encoding::from_alphabet_string(LATIN);
    let cipher_encoding = Encoding::from_alphabet_string(ZODIAC);

    println!("Loading ciphertext");
    let cipher_buf =
        read_encoded_text_from_file("./data/ciphers/z408.txt", &cipher_encoding, HashMap::new());

    println!("Loading frequency data");
    let frequency5 = Frequency::new(
        &stats::Frequency::from_count(&stats::Count::from_file("./data/en_5gram.txt", 5)),
        &output_encoding,
    );
    // let frequency4 = Frequency::new(
    //     &stats::Frequency::from_count(&stats::Count::from_file("./data/en_4gram.txt", 4)),
    //     &output_encoding,
    // );
    let frequency1 = Frequency::new(
        &stats::Frequency::from_count(&stats::Count::from_file("./data/en_1gram.txt", 1)),
        &output_encoding,
    );
    println!("Loading dictionary data");
    // let dict = stats::Dictionary::from_file("./data/en_dict10.txt", &pre);

    let mut letter_distribution = vec![0.0; 26];
    for (key, val) in frequency1.map.iter() {
        letter_distribution[key[0] as usize] = *val;
    }

    use std::cell::RefCell;
    let previous_progress = RefCell::new(0.0f64);

    let cycle = 1000;
    let derive_cycle = 100000;
    let mutate_cycle = 1000;
    let config = Config {
        cycle,
        mutate_cycle,
        derive_cycle,
        report: |climber, count, accepted, rejected| {
            let progress = ((count as f64 / cycle as f64) * 100.0).floor();
            if progress > *previous_progress.borrow() {
                *previous_progress.borrow_mut() = progress;
                let ratio = accepted as f64 / rejected as f64;
                println!(
                    "Progress: {:>3.0}% | Score: {:>8.3} / {:>8.3} | Accept/Reject: {:>6.3}",
                    progress, climber.top_energy, climber.run_energy, ratio
                );
            }
        },
        random_key: |climber| {
            println!("Initializing key");
            climber.run_key.copy(&Key::random_distribution(
                climber.cipher_alphabet.len(),
                &letter_distribution,
            ));
        },
        derive_key: |_climber| {
            // key.random_swap(&state.crib);
        },
        mutate_key: |climber| {
            climber.run_key.random_putc(&climber.output_alphabet);
        },
        crib: |_climber| {
            // key.random_swap(&state.crib);
            // let text = climber.output_encoding.decode_str(&climber.output_buf);
            // let words = stats::find_words(&text, &dict.0);
            // for (i, j) in words.iter() {
            //     if j-i >= 6 {
            //         let word = climber.output_encoding.decode_str(&climber.output_buf[*i..*j]);
            //         for c in *i..*j {
            //             climber.crib_char(climber.cipher_buf[c], climber.output_buf[c]);
            //         }
            //     }
            // }
        },
        energy: |output| {
            let score = frequency5.score(output);
            (score * ENERGY_VALUE) / output.len() as f64
        },
        accept: |prev, next| lavy_accept(prev, next),
    };

    println!("Climbing...");
    let mut climber = Climber::new(
        cipher_buf,
        cipher_encoding.clone(),
        output_encoding.clone(),
        homophone_ratio,
    );
    climber.crib_str(0, "ILIKEKILLINGPEOPLE");
    climber.climb(&config);
    climber.top_key.decode(&climber.cipher_buf, &mut climber.output_buf);

    println!("Key: {:?}", climber.top_key);
    println!("---");
    println!("{}", output_encoding.decode_str(&climber.output_buf));
    println!("---");
}
