use std::{thread, time};

static mut TURN: char = 'a';
static mut IS_A_TRYING_TO_LOCK: bool = false;
static mut IS_B_TRING_TO_LOCK: bool = false;

static mut RESOURCE: i64 = 0;

fn main() {
  unsafe {
    let thread_a_handle = thread::spawn(|| loop {
      TURN = 'b';
      IS_A_TRYING_TO_LOCK = true;

      while TURN == 'b' && IS_B_TRING_TO_LOCK {}

      RESOURCE += 1;

      println!("Thread a: RESOURCE is {:?}", RESOURCE);

      thread::sleep(time::Duration::from_secs(1));

      IS_A_TRYING_TO_LOCK = false;
    });

    let thread_b_handle = thread::spawn(|| loop {
      TURN = 'a';
      IS_B_TRING_TO_LOCK = true;

      while TURN == 'a' && IS_A_TRYING_TO_LOCK {}

      RESOURCE += 1;

      println!("Thread b: RESOURCE is {:?}", RESOURCE);

      thread::sleep(time::Duration::from_secs(1));

      IS_B_TRING_TO_LOCK = false;
    });

    thread_a_handle.join().unwrap();

    thread_b_handle.join().unwrap();
  }
}
