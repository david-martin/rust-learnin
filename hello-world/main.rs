// These are macros!
// https://doc.rust-lang.org/rust-by-example/macros.html
macro_rules! say {
  ($words:expr) => {
    println!("{}", $words);
  }
}

macro_rules! say_hello {
  () => {
    say!("Hello World!");
  }
}

fn main() {
    say_hello!();
    say!("Hello again!");

    // Inferred as &'static str
    // no heap, stored in read-only memory
    let msg = "Hi. Still here...";
    say!(msg);

    // Explicitly typed as &str
    // functionally equivalent to above
    let msg2: &str = "Whatever";
    say!(msg2);

    // A String, which is a heap-allocated, growable string
    let mut msg3: String = String::from("I'm");
    msg3.push_str(" done.");
    say!(msg3);
  }
  