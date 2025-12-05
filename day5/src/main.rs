use std::io::BufRead;

//part 1
fn lecteur(file_name: &str) -> std::io::Result<(Vec<(u64, u64)>, Vec<u64>)> {
    let fichier = std::fs::File::open(file_name)?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut vec_vec: Vec<(u64, u64)> = Vec::new();
    let mut vec: Vec<u64> = Vec::new();
    let mut transition = 0;

    for maybe_line in lecteur.lines() {
        let line = maybe_line?;

        if line.trim().is_empty() {
            transition = 1;
            continue;
        }

        if transition == 0 {
            let tuple = line.split('-').collect::<Vec<_>>();
            vec_vec.push((
                tuple[0].parse::<u64>().unwrap(),
                tuple[1].parse::<u64>().unwrap(),
            ));
        } else {
            vec.push(line.parse::<u64>().unwrap());
        }
    }
    Ok((vec_vec, vec))
}

fn fresh(vec_vec: Vec<(u64, u64)>, vec: Vec<u64>) -> u64 {
    let mut count = 0;
    for i in vec {
        for (a, b) in vec_vec.clone() {
            if i >= a && i <= b {
                count += 1;
                break;
            }
        }
    }
    count
}

//part 2
fn lecteur_vip(file_name: &str) -> std::io::Result<Vec<Vec<u64>>> {
    let fichier = std::fs::File::open(file_name)?;
    let lecteur = std::io::BufReader::new(fichier);
    Ok(lecteur
        .lines()
        .map(|x| {
            x.unwrap()
                .split('-')
                .map(|t| t.parse::<u64>().unwrap())
                .collect()
        })
        .collect())
}

fn modify(vec: &mut Vec<Vec<u64>>) -> usize {
    let mut change = 0;
    for i in 0..vec.len() {
        for j in 0..vec.len() {
            if ((vec[i][0] >= vec[j][0] && vec[i][0] <= vec[j][1])
                || (vec[i][1] >= vec[j][0] && vec[i][1] <= vec[j][1]))
                && (i != j)
            {
                if (vec[i][0] >= vec[j][0] && vec[i][0] <= vec[j][1])
                    && (vec[i][1] >= vec[j][0] && vec[i][1] <= vec[j][1])
                {
                    vec.remove(i);
                    change = 1;
                    break;
                } else if vec[i][0] >= vec[j][0] && vec[i][0] <= vec[j][1] {
                    vec[i] = Vec::from([vec[j][0], vec[i][1]]);
                    vec.remove(j);
                    change = 1;
                    break;
                } else if vec[i][1] >= vec[j][0] && vec[i][1] <= vec[j][1] {
                    vec[i] = Vec::from([vec[i][0], vec[j][1]]);
                    vec.remove(j);
                    change = 1;
                    break;
                }
            }
        }
        if change == 1 {
            break;
        }
    }
    if change == 1 {
        modify(vec);
    }

    0
}

fn main() {
    let mut vec = lecteur_vip("test.txt").unwrap();
    modify(&mut vec);
    let mut res = 0;
    for i in vec {
        res += i[1] - i[0] + 1;
    }
    println!("{:?}", res);
}
