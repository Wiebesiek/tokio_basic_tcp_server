extern crate core;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::{env, thread};
use std::error::Error;
use std::io::ErrorKind;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut counter: u32 = 0;
  let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8080".to_string());

  let listener = TcpListener::bind(&addr).await?;
  println!("Listening on: {}", addr);

  loop {
    counter += 1;

    // Asynchronously wait for an inbound socket.
    let (mut socket, _) = listener.accept().await?;
    tokio::spawn(async move {
      let mut buf = vec![0; 1024];
      loop {
        // simulate some work
        thread::sleep(Duration::from_millis(1000));
        println!("Connection Made - Counter {}", counter);
        let n = socket
          .read(&mut buf)
          .await
          .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::ConnectionAborted {
              0
            } else {
              panic!("Read Connection ended");
            }
          });

        if n == 0 {
          return;
        }

        socket
          .write_all(&buf[0..n])
          .await
          .expect("failed to write data to socket");
      }
    });
  }
}
