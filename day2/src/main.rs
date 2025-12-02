use std::io::BufRead;

fn lecture(file_name: &str) -> std::io::Result<usize> {
    let fichier = std::fs::File::open(file_name)?;
    let lecteur = std::io::BufReader::new(fichier);

    Ok(lecteur
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.to_string())
        .fold(Vec::new(), |mut acc: Vec<Vec<usize>>, x| {
            let tmp = x.split('-').map(|x| x.to_string()).collect::<Vec<_>>();
            acc.append(&mut vec![
                (tmp[0].parse().unwrap()..=tmp[1].parse().unwrap()).collect(),
            ]);

            acc
        })
        .into_iter()
        .fold(0, |acc, x| {
            acc + x.into_iter().fold(0, |mut smaller_acc, val| {
                let chaine = val.to_string();
                let mut test_str;
                let mut stop;
                for i in 1..chaine.len() {
                    test_str = &chaine[0..i];
                    if (chaine.len() % test_str.len()) == 0 {
                        stop = 0;
                        for i in (test_str.len()..chaine.len()).step_by(test_str.len()) {
                            if !(chaine[i..chaine.len()].starts_with(test_str)) {
                                stop = 1;
                                break;
                            }
                        }
                        if stop == 0 {
                            smaller_acc += val;
                            break;
                        }
                    }
                }
                smaller_acc
            })
        }))
}

fn main() {
    let res = lecture("input.txt").unwrap();
    println!("{:?}", res);
}
