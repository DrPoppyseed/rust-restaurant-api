use std::{env::var, error::Error, iter, thread};

type DynError = Result<(), Box<dyn Error>>;

#[tokio::main]
async fn main() -> () {
  let items: Vec<_> = iter::repeat(0).take(5).collect();

  let threads: Vec<_> = items
    .into_iter()
    .enumerate()
    .map(|(i, x)| {
      thread::spawn(move || {
        let thread_id = i + x;
        thread(thread_id.clone())
      })
    })
    .collect();

  for handle in threads {
    handle.join().unwrap().await.unwrap()
  }
}

async fn thread(thread_id: usize) -> DynError {
  let server_addr = format!(
    "{}{}",
    "http://",
    var("ADDR").expect("Error: failed to access ADDR environment variable")
  );

  println!("Thread {} accessing {}", thread_id, server_addr);
  let resp = reqwest::get(server_addr).await?.text().await?;

  println!("{:#?}", resp);
  Ok(())
}
