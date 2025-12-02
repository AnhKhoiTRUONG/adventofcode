use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead, Write};

fn lecture(file_name: String) -> std::io::Result<Vec<String>> {
    let fichier = std::fs::File::open(file_name)?;
    let lecteur = std::io::BufReader::new(fichier);

    lecteur.lines().collect()
}

fn rotate(l: &Vec<String>) -> i32 {
    let mut x: i32 = 50;
    let mut val: i32 = 0;
    let mut zeros: i32 = 0;
    for chaine in l {
        val = chaine[1..chaine.len()].parse().unwrap();
        if let Some(char) = chaine.chars().next() {
            if char == 'L' {
                if x - val <= 0 {
                    if x == 0 {
                        zeros += ((x - val) / 100).abs();
                        println!("at 0: {}", ((x - val) / 100).abs());
                    } else {
                        zeros += (((x - val) / 100) - 1).abs();
                        println!("at 0: {}", (((x - val) / 100) - 1).abs());
                    }
                } else {
                    println!("at 0: {}", 0);
                }
                x = (x - val).rem_euclid(100);
            } else {
                println!("at 0: {}", ((x + val) / 100).abs());
                zeros += ((x + val) / 100).abs();
                x = (x + val).rem_euclid(100);
            }
            println!("x = {x:?}");
        }
    }
    zeros
}

fn main() {
    let x = lecture("input.txt".to_string());
    let a = rotate(&x.unwrap());
    println!("{}", a);
    println!("Hello, world!");
}
