use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use rand::Rng;
use sha2::{Sha256, Digest};

async fn handle_client(mut stream: tokio::net::TcpStream) {
    let mut challenge = [0u8; 16];
    let nonce_length: usize = 2;

    rand::thread_rng().fill(&mut challenge);
    if stream.write_all(&challenge).await.is_err() {
        eprintln!("write to the stream failed");
        return;
    }

    let mut response = [0u8; 16];
    if stream.read_exact(&mut response).await.is_err() {
        eprintln!("read from the stream failed");
        return;
    }

    let mut hasher = Sha256::new();
    hasher.update(&challenge);
    hasher.update(&response);

    let result = hasher.finalize();
    let nonce = &result[0..nonce_length];

    if nonce.iter().all(|x| *x == 0) {
        if stream.write_all(b"Word of Wisdom").await.is_err() {
            eprintln!("write to the stream failed");
        }
    };
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:12346").await?;
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
