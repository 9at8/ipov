//extern crate rand;
extern crate hound;

use std::io;
//use std::cmp::Ordering;
//use rand::Rng;
//use hound;

use std::fs;
use std::i16;
use std::u32;
use std::f32;
use std::f32::consts::PI;

fn main() {
    //let fname = env::args().nth(1).expect("no file given");
    //let mut reader = hound::WavReader::open(&fname).unwrap();
    //unwrap complete
    let mut reader = hound::WavReader::open("bgmTest/thankyou1.wav").unwrap();
    
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    
    let (ts, tu, n) = samples.iter().fold((0.0, 0.0, 0.0), |(ts, tu, n), &s| {
        let signed = s as f64;
        let unsigned = (s as u16) as f64;
        (ts + signed, tu + unsigned, n + 1.0)
    });
    let ms = ts / n;
    let mu = tu / n;
    println!("mean signed:   {} (should be 0, deviation is {})", ms, ms.abs());
    println!("mean unsigned: {} (should be 2^16 - 1, deviation is {})", mu, (mu - 32767.0).abs());

    let (ts, tu) = samples.iter().fold((0.0, 0.0), |(ts, tu), &s| {
        let ds = s as f64 - ms;
        let du = (s as u16) as f64 - mu;
        (ts + ds * ds, tu + du * du)
    });
    let rmss = (ts / n).sqrt();
    let rmsu = (tu / n).sqrt();
    println!("rms signed:    {}", rmss);
    println!("rms unsigned:  {}", rmsu);

    let sqr_sum = reader.samples::<i16>()
                        .fold(0.0, |sqr_sum, s| {
        let sample = s.unwrap() as f64;
        sqr_sum + sample * sample
    });
    println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
    println!("len is {}", (reader.len() as u32));
    
    /*
    let mut reader = hound::WavReader::open("bgmTest/thankyou1.wav").unwrap();
    let sqr_sum = reader.samples::<i16>()
                        .fold(0.0, |sqr_sum, s| {
        let sample = s.unwrap() as f64;
        sqr_sum + sample * sample
    });
    println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
    */
    /*
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number); //comment out if not debugging

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //let guess: u32 = guess.trim().parse()
        //    .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }*/
}

