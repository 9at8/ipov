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
    let mut reader = hound::WavReader::open("sine.wav").unwrap();
    
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
