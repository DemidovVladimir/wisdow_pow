use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::thread;
use rand::Rng;
use sha2::{Sha256, Digest};

fn handle_client<T: Read + Write>(stream: &mut T) {
    let mut challenge = [0u8; 16];
    let nonce_length: usize = 2;

    rand::thread_rng().fill(&mut challenge);
    stream.write(&challenge).expect("write to the stream failed");

    let mut response = [0u8; 16];
    stream.read(&mut response).expect("read from the stream failed");

    let mut hasher = Sha256::new();
    hasher.update(&challenge);
    hasher.update(&response);

    let result = hasher.finalize();
    let nonce = &result[0..nonce_length];

    if nonce.iter().all(|x| *x == 0) {
        stream.write(b"Word of Wisdom").expect("write to the stream failed");
    };
}

fn main() -> Result<(), io::Error>{
    let listener = TcpListener::bind("0.0.0.0:12346")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    handle_client(&mut stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_handle_client() {
        let challenge = [0u8; 16];
        let response = [0u8; 16];
        let mut stream = Cursor::new(Vec::new());

        stream.write(&response).unwrap();
        stream.write(&challenge).unwrap();
        stream.set_position(0);

        handle_client(&mut stream);

        let mut expected_response = Vec::new();
        expected_response.extend_from_slice(&challenge);
        expected_response.extend_from_slice(b"Word of Wisdom");

        assert_eq!(expected_response, stream.into_inner());
    }
}

