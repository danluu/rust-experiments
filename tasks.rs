use core::task::spawn;

fn main() {
    fn print_message() { println("I am running in a different task!"); }
    spawn(print_message);
}
