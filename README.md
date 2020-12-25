There are two threads running infinite loops:

```rust
 let thread_a_handle = thread::spawn(...);
 let thread_b_handle = thread::spawn(...);
```

They are both competing for the same resource:

```rust
static mut RESOURCE: i64 = 0;
```

This variable tells whose turn it is:

```rust
static mut TURN: char = 'a';
```

Each thread passes the turn to the other thread at the beginning of the loop

```rust
let thread_a_handle = thread::spawn(|| loop {
      TURN = 'b';
      IS_A_TRYING_TO_LOCK = true;
    });
```

Each thread has a variable to show that it wants to acquire the lock

```rust
static mut IS_A_TRYING_TO_LOCK: bool = false;
static mut IS_B_TRING_TO_LOCK: bool = false;
```

The thread blocks and waits for its turn

```rust
  let thread_a_handle = thread::spawn(|| loop {
      TURN = 'b';
      IS_A_TRYING_TO_LOCK = true;

      while TURN == 'b' && IS_B_TRING_TO_LOCK {}
    });
```

After acquiring the lock, the thread will do what is needed

```rust
   let thread_a_handle = thread::spawn(|| loop {
      TURN = 'b';
      IS_A_TRYING_TO_LOCK = true;

      while TURN == 'b' && IS_B_TRING_TO_LOCK {}

      RESOURCE += 1;

      println!("Thread a: RESOURCE is {:?}", RESOURCE);

      thread::sleep(time::Duration::from_secs(1));
    });
```

After it is done, it sets `IS_A_TRYING_TO_LOCK` so thread B can acquire the lock

```rust
   let thread_a_handle = thread::spawn(|| loop {
      TURN = 'b';
      IS_A_TRYING_TO_LOCK = true;

      while TURN == 'b' && IS_B_TRING_TO_LOCK {}

      RESOURCE += 1;

      println!("Thread a: RESOURCE is {:?}", RESOURCE);

      thread::sleep(time::Duration::from_secs(1));

      IS_A_TRYING_TO_LOCK = false;
    });
```
