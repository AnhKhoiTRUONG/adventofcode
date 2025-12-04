use std::io::BufRead;

struct Hehe {
    tab: Vec<Vec<char>>,
    hauteur: i32,
    largeur: i32,
}

impl Hehe {
    fn new() -> Hehe {
        Hehe {
            tab: Vec::new(),
            hauteur: 0,
            largeur: 0,
        }
    }
}

fn lecteur(file_name: &str) -> std::io::Result<Hehe> {
    let fichier = std::fs::File::open(file_name)?;
    let lecteur = std::io::BufReader::new(fichier);
    let mut hehe = Hehe::new();
    hehe.tab = lecteur
        .lines()
        .map(|maybe_string| maybe_string.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    hehe.largeur = hehe.tab[0].len() as i32;
    hehe.hauteur = hehe.tab.len() as i32;
    Ok(hehe)
}

//part 1
fn detecte(mut hehe: Hehe) -> usize {
    let mut count = 0;
    let mut adj;
    for i in 0..hehe.hauteur {
        for j in 0..hehe.largeur {
            if hehe.tab[i as usize][j as usize] == '@' {
                adj = 0;
                for x in i - 1..=i + 1 {
                    for y in j - 1..=j + 1 {
                        // println!("test:i = {}, j ={}, x = {}, y = {}", i, j, x, y);
                        if (x != i || y != j)
                            && x >= 0
                            && y >= 0
                            && x < hehe.hauteur
                            && y < hehe.largeur
                            && hehe.tab[x as usize][y as usize] != '.'
                        {
                            // println!("rentrer: i = {}, j ={}, x = {}, y = {}", i, j, x, y);
                            adj += 1;
                        }
                    }
                }
                if adj < 4 {
                    hehe.tab[i as usize][j as usize] = 'x';
                    count += 1;
                }
            }
        }
    }
    // println!("{:?}", hehe.tab);
    count
}

//part 2
fn detecte_mut(hehe: &mut Hehe, count_arg: usize) -> usize {
    let mut count = 0;
    let mut liste_changed = Vec::new();
    let mut adj;
    for i in 0..hehe.hauteur {
        for j in 0..hehe.largeur {
            if hehe.tab[i as usize][j as usize] == '@' {
                adj = 0;
                for x in i - 1..=i + 1 {
                    for y in j - 1..=j + 1 {
                        if (x != i || y != j)
                            && x >= 0
                            && y >= 0
                            && x < hehe.hauteur
                            && y < hehe.largeur
                            && hehe.tab[x as usize][y as usize] == '@'
                        {
                            adj += 1;
                        }
                    }
                }
                if adj < 4 {
                    liste_changed.push((i, j));
                    count += 1;
                }
            }
        }
    }
    for (i, j) in liste_changed {
        hehe.tab[i as usize][j as usize] = '.';
    }
    if count == 0 {
        count_arg
    } else {
        detecte_mut(hehe, count_arg + count)
    }
}

fn main() {
    let mut hehe = lecteur("test.txt").unwrap();
    println!("{:?}", detecte_mut(&mut hehe, 0));
}
