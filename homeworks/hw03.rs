fn main() {
    const WIDTH: usize = 15;
    const HEIGHT: usize = 9;
    let mut result = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Малюємо рамку та діагоналі
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 || x == y || x == WIDTH - 1 - y {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
    println!("Done!");
}
