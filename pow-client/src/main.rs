use std::io::{self, Read, Write};
use std::net::TcpStream;
use sha2::{Sha256, Digest};
use rand::Rng;

fn main() -> Result<(), io::Error> {
    // Uses service config from docker-compose.yml
    let mut stream = TcpStream::connect("server:12346")?;
    // The complexity is determined by this bytes array and nonce length
    let mut challenge = [0u8; 16];
    let nonce_length: usize = 2;

    // Read the challenge from the server
    stream.read(&mut challenge)?;

    let mut response = [0u8; 16];
    loop {
        rand::thread_rng().fill(&mut response);

        // Use hash builder to generate hash to further validate the nonce
        let mut hasher = Sha256::new();
        hasher.update(&challenge);
        hasher.update(&response);
        let result = hasher.finalize();

        // Check if the nonce is valid
        let nonce = &result[0..nonce_length];
        if nonce.iter().all(|x| *x == 0) {
            println!("Found a solution!");
            break;
        }
    }

    stream.write(&response)?;
    let mut result = [0u8; 16];
    stream.read(&mut result)?;
    println!("{}", String::from_utf8_lossy(&result));
    Ok(())
}
