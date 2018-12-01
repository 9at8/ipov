//extern crate rand;
extern crate hound;

//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;
//use hound;

fn main() {

    let mut reader = hound::WavReader::open("bgmTest/thankyou1.wav").unwrap();
    let sqr_sum = reader.samples::<i16>()
                        .fold(0.0, |sqr_sum, s| {
        let sample = s.unwrap() as f64;
        sqr_sum + sample * sample
    });
    println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());

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

