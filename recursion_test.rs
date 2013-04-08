fn f(){
    fn g() {
        println("g");
    }
    println("f");
    g();
}

fn main() {
    f();
}
