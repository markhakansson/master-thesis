// Aperiodic task that does some work everytime the
// button is pressed
#[task(binds = EXTI15_10,
       priority = 2,
       resources = [button, shared_u8, shared_u16, dwt])]
fn button_click(mut cx: button_click::Context) {
    // Clear interrupt
    cx.resources
        .button
        .lock(|button| button.clear_interrupt_pending_bit());

    // Some nonsensical work here...
    cx.resources.shared_u8.lock(|i| *i += 1);
    let value: u32 = cx.resources.dwt.lock(|dwt| {
        dwt.cyccnt.read()
    });
    if value == 123456789 {
        asm::delay(10_000);
    }
    cx.resources.shared_u16.lock(|i| *i += 3);
}
