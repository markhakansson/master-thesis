#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    // imports here
    use cortex_m;

    #[resources]
    struct Resources {
        // shared resources here
    }

    #[init]
    fn init(mut cx: init::Context) -> (init::LateResources, init::Monotonics) {
        // Initialize hardware and resources here
    }

    #[task]
    fn task(mut cx: task::Context) {
        // a user task
    }

    #[idle]
    fn idle(_: idle::Context) {
        // idle function
    }
}
