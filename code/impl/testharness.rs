// Two simplified software tasks
#[task(resources = [a])]
fn t1(_: t1::Context) {}
#[task]
fn t2(_: t2::Context) {}

fn main() {
    // Make the integer controlling the flow symbolic
    let mut __klee_task_id: u8 = 0;
    klee_make_symbolic(&mut __klee_task_id, "id");
    match __klee_task_id {
        0u8 => {
            // Test if the type of `a` can be set symbolic
            if supported_type(resources.a) {
                klee_make_symbolic(&mut resource.a, "a\n");
            }
            // Then call the task associated with this
            // number and resources
            t1();
        }
        1u8 => {
            t2();
        }
        _ => (),
    }
}
