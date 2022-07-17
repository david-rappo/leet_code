// Position (n) = 0 1 2 3 4 5
// Values =       0 1 1 2 3 5
#[allow(dead_code)]
pub fn fibonacci_number(n: i32) -> i32 {
    let begin = 2;
    if n < begin {
        return n;
    }

    let mut x = 0;
    let mut y = 1;
    let end = n + 1;
    for _ in begin..end {
        let next = x + y;
        x = y;
        y = next;
    }

    y
}