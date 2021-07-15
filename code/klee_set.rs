fn main() {
    let mut input: u32 = 0;
    klee_make_symbolic(&mut input, "input\n");
    function(input);
}
