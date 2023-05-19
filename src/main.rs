use rand::{seq::SliceRandom, SeedableRng};
use rand::rngs::StdRng;
use sha3::{Digest, Sha3_256};
use std::collections::HashSet;
use std::io::{self, Write};
use std::collections::HashMap;
use clap::Parser;
use image::{ImageBuffer, Rgba};
use chrono::prelude::*;

// generate the image  
//TODO// icon at index 10n+1 causes thread panic due to image buffer dimension overflow
//TODO// because no implementation of wrapping around to the next row
fn img_generator(txt2dcrypt: &str, s2p_map: &HashMap<char, Vec<char>>) {
    let width = 2000;
    let height = 3000;
    let icon_width = 200;
    let icon_height = 300;
    let grid_size = 100;

    let mut image = ImageBuffer::<Rgba<u8>, _>::new(width, height);

    for (index, icon_index) in (0..).zip((0..txt2dcrypt.len()).step_by(6)) {
        let icon = &txt2dcrypt[icon_index..icon_index + 6];
        let x = (index % grid_size) * icon_width;
        let y = (index / grid_size) * icon_height;

        for (i, c) in icon.chars().enumerate() {
            let icon_color = match c {
                'c' => Rgba([0, 255, 255, 255]),    // cyan
                'y' => Rgba([255, 255, 0, 255]),    // yellow
                'm' => Rgba([255, 0, 255, 255]),    // magenta
                'r' => Rgba([255, 0, 0, 255]),      // red
                'g' => Rgba([0, 255, 0, 255]),      // green
                'b' => Rgba([0, 0, 255, 255]),      // blue
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

    let timestamp = Local::now().format("%y%m%d%H%M%S");
    let filename = format!("{}.png", timestamp);
    image.save(&filename).expect("Failed to save image");
}



// generate all 720 permutations of 'chars'
fn permutation_generator(chars: &[char], used: &mut HashSet<char>, permutor: &mut Vec<char>, permutations: &mut Vec<Vec<char>>) {
    if permutor.len() == chars.len() {
        permutations.push(permutor.clone());
        return;
    }

    for c in chars {
        if used.insert(*c) {
            permutor.push(*c);
            permutation_generator(chars, used, permutor, permutations);
            permutor.pop();
            used.remove(c);
        }
    }
}


/// hh666, dynamic pseudorandom 1:1 substitution cipher
#[derive(Parser, Debug)]
#[command(name = "hh666", author = "funkelman", version = "0.0.0", about, long_about = None)]
struct Args {
    /// UserInputStringKEY
    #[arg(short, long)]
    key: Option<String>,

    /// UserInputStringTXT2NCRYPT
    #[arg(short, long,)]
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //??// initializing clap_parser or smthng idfk
    let args = Args::parse();
    println!("parsed_arguments: \n{:?}", args);


    // read UserInputString "k"
    let k = args.key.map(|s| s.trim_end().to_owned())
    .unwrap_or_else(|| {
        println!("please define k!");
        print!("k: ");
        io::stdout().flush().unwrap();
        let mut k = String::new();
        match io::stdin().read_line(&mut k) {
            Ok(_) => k.trim_end().to_owned(),
            Err(err) => {
                eprintln!("Error reading UserInputStringKey: {}", err);
                std::process::exit(1);
            }
        }
    });


    // vector of symbols to map
    let symbols = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_', '.', ',',
        ';', ':', '!', '?', '&', '@', '#', '$', '§', '≈', '≠', '=', '<',
        '>', '~', '–', '-', '±', '+', '’', '”', '*', '°', '^', '/', '%',
        '(', ')', '[', ']', '{', '}', '|', '\t', ' ', '\n'
    ];


    // hash input string "k" w/ SHA3-256
    let k_hash = Sha3_256::digest(k.as_bytes());

    //[DBG]// print "k_hash"
    println!("k_hash: {:x}", k_hash);

    // use "k_hash" as seed for the PRNG
    let seed: [u8; 32] = k_hash[..32].try_into()?;
    let mut prng = StdRng::from_seed(seed);

    // generate all 720 permutations of "chars" -> "permutations"
    let chars = ['r', 'g', 'b', 'c', 'y', 'm'];
    let mut used = HashSet::new();
    let mut permutor = Vec::new();
    let mut permutations = Vec::new();
    permutation_generator(&chars, &mut used, &mut permutor, &mut permutations);

    // shuffle the 720 permutations in "permutations" using the PRNG -> "p_pr_720"
    permutations.shuffle(&mut prng);

    // select the 101 pseudorandom elements from the 720 permutations -> "p_pr_101"
    let mut selection = permutations.into_iter().take(101).collect::<Vec<_>>();
    selection.shuffle(&mut prng);

    //[DBG]// print the 101 pseudorandom selected elements "p_pr_101"
    //println!("p_pr_101: ");
    //for p in &selection {
    //    println!("{}", p.iter().collect::<String>());
    //}

    // create a hashmap to store p_pr_101 as values with s_101 as keys
    let mut s2p_map: HashMap<char, Vec<char>> = HashMap::new();

    // iterate through the symbols and assign a selected permutation to each one
    for (symbol, permutation) in symbols.iter().zip(selection.iter()) {
    s2p_map.insert(*symbol, permutation.to_owned());
    }

    //[DBG]// print the resulting hashmap "s2p_map"
    println!("s2p_map:");
    for (symbol, permutation) in s2p_map.iter() {
        println!("{}: {}", symbol, permutation.iter().collect::<String>());
    }

    // read "txt2ncrypt"
    let txt2ncrypt = args.input.map(|s| s.trim_end().to_owned())
    .unwrap_or_else(|| {
        println!("please define txt2ncrypt!");
        print!("txt2ncrypt: ");
        io::stdout().flush().unwrap();
        let mut txt2ncrypt = String::new();
        match io::stdin().read_line(&mut txt2ncrypt) {
            Ok(_) => txt2ncrypt.trim_end().to_owned(),
            Err(err) => {
                eprintln!("Error reading UserInputStringTXT2NCRYPT: {}", err);
                std::process::exit(1);
            }
        }
    });
    

    // encrypt "txt2ncrypt" using "s2p_map"
    let txt2dcrypt = txt2ncrypt.chars()
    .map(|c| {
        s2p_map.get(&c).unwrap().iter().collect::<String>()
    })
    .collect::<String>();

    // print "txt2dcrypt"
    println!("txt2dcrypt: {}", txt2dcrypt);

    img_generator(&txt2dcrypt, &s2p_map);

Ok(())
}

//Z85
//f!j$wgcoR9x!r!ng&9