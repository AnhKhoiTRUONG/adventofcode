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

//use this for part 1
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

//use this for part 2
fn meilleur_gain(prix: Vec<u32>) -> u64 {
    // let mut max_gain = 0;
    let mut gain: u64 = 0;
    let mut i = 0;
    let mut number_completed = 0;
    let mut places = prix.len();
    let mut max_ind;
    let mut max;
    while i < prix.len() && number_completed != 12 {
        if places == (12 - number_completed) {
            gain = gain * 10 + prix[i] as u64;
            println!("max int = {}", prix[i]);
            i += 1;
            number_completed += 1;

            places -= 1;
        } else {
            max = prix[i];
            max_ind = i;
            for t in i..i + (places - (12 - number_completed) + 1) {
                if prix[t] > max {
                    max = prix[t];
                    max_ind = t;
                }
            }
            gain = gain * 10 + prix[max_ind] as u64;
            number_completed += 1;
            places = prix.len() - (max_ind + 1);
            i = max_ind + 1;
        }
    }
    gain
}

fn main() {
    println!("{:?}", lecture("input.txt".to_string()));
}
