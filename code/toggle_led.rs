// Periodic task that toggles the LED on the defined interval
#[task(binds = TIM2,
       priority = 1,
       resources = [led, led_on, timer, shared_u8, shared_u16])
]
fn toggle_led(mut cx: toggle_led::Context) {
    // Clear interrupt
    cx.resources.timer.lock(|timer| {
        timer.clear_interrupt(Event::TimeOut);
    });

    let mut su8 = cx.resources.shared_u8;
    let mut su16 = cx.resources.shared_u16;

    // Check the shared resources and
    // do some work here in rare cases
    su8.lock(|su8| {
        su16.lock(|su16| {
            if *su8 == 123 {
                asm::delay(1_000);
                if *su16 == 12345 {
                    asm::delay(10_000);
                    *su16 += 10;
                }
            }
        });
    });

    // Toggle the LED depending on current status
    let powered_on = cx.resources.led_on.lock(|led_on| *led_on);
    if !powered_on {
        cx.resources.led.lock(|led| {
            led.set_high().unwrap();
        });
        cx.resources.led_on.lock(|led_on| *led_on = true);
    } else {
        cx.resources.led.lock(|led| {
            led.set_low().unwrap();
        });
        cx.resources.led_on.lock(|led_on| *led_on = false);
    }
}
