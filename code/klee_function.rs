fn function(integer: u32) -> u32 {
    let mut value = integer;

    if value < 2000 {
        value = value + 1000;
        if value == 1234 {
            panic!("invalid number");
        }
        if value == 5000 {
            panic!("this can never happen");
        }
        return value;
    } else {
        return value;
    }
}
