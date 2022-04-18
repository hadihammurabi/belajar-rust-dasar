use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn run(workers: i32) {
  let mut handles: Vec<JoinHandle<()>> = vec![];
  for i in 0..workers {
    let handle = thread::spawn(move || {
      worker(i + 1)
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
}

pub fn worker(id: i32) {
  thread::sleep(Duration::from_millis(1000));
  println!("Thred {} is finished.", id)
}
