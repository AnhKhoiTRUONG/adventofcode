use core::num;
use std::io::BufRead;

fn lecture(file_name: String) -> std::io::Result<u64> {
    let fichier = std::fs::File::open(file_name)?;
    let lecteur = std::io::BufReader::new(fichier);

    Ok(lecteur
        .lines()
        .map(|maybe_string| {
            maybe_string
                .unwrap()
                .chars()
                .map(|x| x as u32 - 48)
                .collect::<Vec<u32>>()
        })
        .fold(0, |acc, x| acc + meilleur_gain(x)))
}

//part 1
fn meilleur_gain_naif(prix: Vec<u32>) -> u32 {
    let mut max_gain = 0;
    let mut gain;
    for i in 0..(prix.len() - 1) {
        gain = prix[i] * 10 + prix[i + 1..prix.len()].iter().max().unwrap();
        if gain > max_gain {
            max_gain = gain;
        }
    }
    println!("max_gain = {}", max_gain);
    max_gain
}

//part 2
fn meilleur_gain(prix: Vec<u32>) -> u64 {
    // let mut max_gain = 0;
    let mut gain: u64 = 0;
    let mut i = 0;
    let mut number_completed = 0;
    let mut places = prix.len();
    let mut max_ind;
    let mut max;
    while i < prix.len() && number_completed != 12 {
        println!(
            "places = {}, number_completed ={}",
            places,
            12 - number_completed
        );
        if places == (12 - number_completed) {
            gain = gain * 10 + prix[i] as u64;
            println!("max int = {}", prix[i]);
            i += 1;
            number_completed += 1;

            places -= 1;
        } else {
            println!(
                "range from {} to {}",
                i,
                i + (places - (12 - number_completed) + 1)
            );
            max = prix[i];
            max_ind = i;
            for t in i..i + (places - (12 - number_completed) + 1) {
                if prix[t] > max {
                    max = prix[t];
                    max_ind = t;
                }
            }
            println!("max ind = {} et max_int = {}", max_ind, prix[max_ind]);
            gain = gain * 10 + prix[max_ind] as u64;
            number_completed += 1;
            places = prix.len() - (max_ind + 1);
            i = max_ind + 1;
        }
    }

    println!("max_gain = {}", gain);
    gain
}

fn main() {
    // println!(
    //     "{:?}",
    //     meilleur_gain(Vec::from([
    //         9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9
    //     ]))
    // );
    println!("{:?}", lecture("input.txt".to_string()));
}
