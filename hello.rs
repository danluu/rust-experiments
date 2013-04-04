fn main() {
    io::println("hello?");
    unique_ptr();
    heap_alloc();
}

fn unique_ptr() {
    let x: ~int = ~1024;
    println(fmt!("unique_ptr: %d", *x));
}

fn heap_alloc() {
    let x: @int = @512;
    use_heap_alloc(x);
    println(fmt!("heap_alloc: %d", *x));   
}

fn use_heap_alloc(x: @int) {
    let y: @int = x;
    println(fmt!("use_heap_alloc: %d", *y));   
}
