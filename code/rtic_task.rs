// Access to shared resources needs to be
// explicitly declared
#[task(resources = [input])]
fn task(mut cx: task::Context) {
    // Lock the `input` resource to
    // gain exclusive access to it
    cx.input.lock(|input| {
        // Read the input pin
        // (discarding the value)
        input.read();
    });
}
