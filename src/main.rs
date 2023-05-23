use rand::{seq::SliceRandom, SeedableRng};
use rand::rngs::StdRng;
use sha3::{Digest, Sha3_256};
use std::collections::HashSet;
use std::io::{self, Write};
use std::collections::HashMap;
use clap::Parser;
use image::{ImageBuffer, Rgba};
use chrono::prelude::*;


//  function to generate icon image 'yymmddHMS.png' with 'txt2ncrypt' and 's2p_map'
fn img_generator(txt2dcrypt: &str, s2p_map: &HashMap<char, Vec<char>>) {
    let width = 2000;
    let height = 3000;
    let icon_width = 200;
    let icon_height = 300;
    let grid_size = 100;

    let mut image = ImageBuffer::<Rgba<u8>, _>::new(width, height);

    for (index, icon_index) in (0..).zip((0..txt2dcrypt.len()).step_by(6)) {
        if index >= grid_size {
            panic!("Too many icons to fit in the image buffer.");
        }

        let slice_end = std::cmp::min(icon_index + 6, txt2dcrypt.len());
        let icon = &txt2dcrypt[icon_index..slice_end];
        let row = index / 10;
        let col = index % 10;

        let x = col * icon_width;
        let y = row * icon_height;

        for (i, c) in icon.chars().enumerate() {
            let icon_color = match c {
                '0' => Rgba([0, 0, 0, 255]),          // 0
                '1' => Rgba([255, 255, 255, 255]),  // 1
                _ => panic!("Invalid icon character"),
            };

            let icon_x = x + (i % 2) * (icon_width / 2);
            let icon_y = y + (i / 2) * (icon_height / 3);

            for y_offset in 0..(icon_height / 3) {
                for x_offset in 0..(icon_width / 2) {
                    let px = icon_x + x_offset;
                    let py = icon_y + y_offset;
                    image.put_pixel(px as u32, py as u32, icon_color);
                }
            }
        }
    }

    if txt2dcrypt.len() > grid_size * 6 {
        panic!("Too many icons to fit within the image buffer.");
    }

    let timestamp = Local::now().format("%y%m%d%H%M%S");
    let filename = format!("{}.png", timestamp);
    image.save(&filename).expect("Failed to save image");
}



//  generate all 65 combinations of 'chars'
fn generate_combinations() -> Vec<String> {
    let characters = vec!['0', '1'];
    let mut combinations = Vec::new();

    for c1 in &characters {
        for c2 in &characters {
            for c3 in &characters {
                for c4 in &characters {
                    for c5 in &characters {
                        for c6 in &characters {
                            let combination = format!("{}{}{}{}{}{}", c1, c2, c3, c4, c5, c6);
                            combinations.push(combination);
                        }
                    }
                }
            }
        }
    }

    combinations
}







fn main() -> Result<(), Box<dyn std::error::Error>> {


    //  read UserInputString "k"
    let mut k = String::new();
    print!("k: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut k)?;
    


    //  vector of the symbols to map
    let symbols = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ', '.', ',',
    ];


    //  hash input string "k" w/ SHA3-256
    let k_hash = Sha3_256::digest(k.as_bytes());

    //[DBG]//   print "k_hash"
    //println!("k_hash: {:x}", k_hash);

    //  use "k_hash" as seed for the PRNG
    let seed: [u8; 32] = k_hash[..32].try_into()?;
    let mut prng = StdRng::from_seed(seed);










    //  generate all 65 combinations of "chars" -> "combinations"
    let mut combinations = generate_combinations();


    //  shuffle the 65 combinations in "combinations" using the PRNG -> "p_pr_65"
    combinations.shuffle(&mut prng);

    //[DBG]//   print the 65 pseudorandom selected elements "p_pr_65"
    //println!("p_pr_65: ");
    //for p in &combinations {
    //    println!("{}", p.chars().collect::<String>());
    //}

    //  create a hashmap to store p_pr_65 as values with s_65 as keys
    let mut s2p_map: HashMap<char, Vec<char>> = HashMap::new();

    //  iterate through the symbols and assign a selected combination to each one
    for (symbol, combination) in symbols.iter().zip(combinations.iter()) {
        let combination_chars: Vec<char> = combination.chars().collect();
        s2p_map.insert(*symbol, combination_chars);
    }

    //[DBG]// print the resulting hashmap "s2p_map"
    println!("s2p_map:");
    for (symbol, combination) in s2p_map.iter() {
        println!("{}: {}", symbol, combination.iter().collect::<String>());
    }



    // Read "txt2ncrypt" and validate if it contains only symbols from the symbols vector
    let mut txt2ncrypt = String::new();
    loop {
        print!("txt2ncrypt: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut txt2ncrypt)?;

        let txt2ncrypt_chars: HashSet<char> = txt2ncrypt.chars().collect();
        let symbols_set: HashSet<char> = symbols.iter().cloned().collect();

        let mut foreign_chars: HashSet<char> = txt2ncrypt_chars.difference(&symbols_set).cloned().collect();
        foreign_chars.remove(&'\n');

        if foreign_chars.is_empty() {
            break;
        } else {
            println!("Invalid characters found: {:?}", foreign_chars);
            txt2ncrypt.clear();
        }
    }

    

    //  encrypt "txt2ncrypt" using "s2p_map"
    let txt2dcrypt = txt2ncrypt.chars()
    .map(|c| {
        s2p_map.get(&c).map(|combination| combination.iter().collect::<String>()).unwrap_or("".to_string())
    })
    .collect::<String>();

    //  print "txt2dcrypt"
    println!("txt2dcrypt: {}", txt2dcrypt);


    //  call 'img_generator' with 'txt2dcrypt' to create icon image 'img2dcrypt' and save it 
    img_generator(&txt2dcrypt, &s2p_map);

Ok(())
}