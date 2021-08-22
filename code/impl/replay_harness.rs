#[task(binds = UART1, resources = [a])]
fn t1(_: t1::Context) {
    // Notify RAUK that a resource is about to be claimed
    bkpt(3);
    t1::resources.a.lock(|a| {});
    // Notify RAUK that a resource has been released
    bkpt(252);
}

// Simplified task handler. The task handler is normally
// executed when an external interrupt is triggered.
fn UART1() {
    // Notify RAUK that a hardware task is being called
    bkpt(2);
    t1();
    // Notify RAUK that a hardware task is finishing
    bkpt(253);
}

fn main() {
    // Initialize program
    init();
    // Match arm controller
    let mut __klee_task_id: u8 = 0;
    // Loop such that RAUK can measure all test vectors
    loop {
        // Breakpoint with argument 255 to notify RAUK when
        // a new test vector should be measured
        bkpt(255);
        match __klee_task_id {
            0u8 => {
                UART1();
            }
            _ => (),
        }
    }
}
