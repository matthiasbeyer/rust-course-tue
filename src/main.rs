fn main() {
    let mut x = 0;

    loop {
        println!("{x}");
        x += 1;

        if x == 10 {
            break;
        }
    }
}
