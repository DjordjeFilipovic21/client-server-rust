use std::net::TcpStream;
use std::io::{self, Write, Read};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    let mut input = String::new();
    loop {
            input.clear();
            println!("Odaberite duzinu reci (3,5,6): ");
            io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();

            if trimmed == "exit" {
                break;
            }

            let number: i32 = match trimmed.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Unos nije validan broj. Pokušajte ponovo.");
                    continue;
                }
            };

        stream.write_all(&number.to_be_bytes())?;
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;
        println!("{}", String::from_utf8_lossy(&buffer[..n]));


        loop{
            let mut input = String::new();
            println!("Unesite slovo: ");
            io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();
            stream.write_all(&trimmed.as_bytes())?;
            stream.flush()?;
            buffer = [0; 1024];
            stream.read(&mut buffer)?;
            println!("{}", String::from_utf8_lossy(&buffer))

        }
    }

    Ok(())
}