use std::sync::mpsc;
use std::thread;

fn worker(tx: &mpsc::Sender<String>, num: i32) {
  let maybe_res = reqwest::blocking::get(format!("https://my-json-server.typicode.com/hadihammurabi/flutter-webservice/contacts/{}", num));
  let res = maybe_res.unwrap();
  let maybe_text = res.text().unwrap();
  tx.send(maybe_text).unwrap();
}

pub fn run() {
  let ids = vec![1,2,3];
  let (tx, rx) = mpsc::channel::<String>();

  for &i in &ids {
    let tx1 = tx.clone();
    thread::spawn(move || {
      worker(&tx1, i);
    });
  }

  for _ in ids {
    let valout = rx.recv().unwrap();
    println!("{}", valout);
  }
}
