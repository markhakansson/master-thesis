// Resources that can be shared by RTIC tasks
#[resources]
struct Resources {
    // Initialize `integer` to 0 at compile-time
    #[init(0)]
    integer: u8,
    // `input` is a hardware peripheral and needs to
    // be initialized at run-time
    input: gpioc::PC13<Input<PullDown>>,
}
