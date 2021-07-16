#[task(resources = [a])]
fn t1() {}
#[task]
fn t2() {}

fn main() {
    let mut __klee_task_id: u8 = 0;
    match __klee_task_id {
        0u8 => {
            if supported_type(resources.a) {
                klee_make_symbolic(&mut resource.a, "a\n");
            }
            t1();
        }
        1u8 => {
            t2();
        }
        _ => (),
    }
}
