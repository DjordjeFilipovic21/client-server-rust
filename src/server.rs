use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use rand::seq::IteratorRandom;

const WORDLIST: [&str; 9] = [
    "vuk",
    "pas",
    "rak",
    "tigar",
    "mačka",
    "ptica",
    "žirafa",
    "hijena",
    "kamila",
];
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection closed by client
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                // Parse the word length sent by the client
                let number = i32::from_be_bytes(buffer[..4].try_into().unwrap());
                println!("Received number: {}", number);

                // Validate word length and select a random word
                let words = WORDLIST
                    .iter()
                    .filter(|word| word.len() == number as usize)
                    .collect::<Vec<_>>();
                let word = words[0];
                if word.is_empty() {
                    stream.write_all(b"Invalid word length").unwrap();
                    continue;
                }

                let mut guess = "_".repeat(word.len());
                let mut remaining_attempts = number * 2;
                stream.write_all(guess.as_bytes()).unwrap();

                // Game loop
                while remaining_attempts > 0 {
                    buffer = [0; 1024];
                    // Send the current state of the word to the client

                    // Read the client's guess
                    let mut input = [0; 1024];
                    let bytes_read = stream.read(&mut input).unwrap();
                    if bytes_read == 0 {
                        println!("Client disconnected");
                        break;
                    }

                    let client_guess = String::from_utf8_lossy(&input[..bytes_read]).trim().to_string();
                    if client_guess == "exit" {
                        println!("Client exited the game");
                        break;
                    }

                    // Process the client's guess
                    if word.contains(&client_guess) {
                        for (i, c) in word.chars().enumerate() {
                            if c.to_string() == client_guess {
                                guess.replace_range(i..i + 1, &client_guess);
                            }
                        }
                        if (guess.eq(word)){
                            stream.write_all(format!("Congratulations! You guessed the word: {}", word).as_bytes()).unwrap();
                            stream.flush().unwrap();
                            break;
                        }
                        stream.write_all(format!("Correct guess! {}", guess).as_bytes()).unwrap();
                        stream.flush().unwrap();

                    } else {
                        stream.write_all(format!("Wrong guess! {}", guess).as_bytes()).unwrap();
                        stream.flush().unwrap();

                    }

                    // Check if the word is fully guessed
                    if guess.eq(word) {
                        stream.write_all(b"Congratulations! You guessed the word!").unwrap();
                        stream.read(&mut buffer).unwrap();
                        break;
                    }

                    remaining_attempts -= 1;
                }

                // End of game
                if remaining_attempts == 0 && !guess.eq(word) {
                    stream.write_all(format!("Game over! The word was: {}", word).as_bytes())
                        .unwrap();
                    stream.flush().unwrap();

                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {}", stream.peer_addr()?);
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}