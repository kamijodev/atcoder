use proconio::input;

fn main() {
    input! {
        h: i32,
        _w: i32,
        arr: [String; h]
    }
    let mut x1 = 999;
    let mut x2 = 999;
    let mut y1 = 999;
    let mut y2 = 999;
    for (h_i, row) in arr.iter().enumerate() {
        for (w_i, col) in row.chars().enumerate() {
            if col == 'o' && x1 == 999 {
                x1 = w_i;
                y1 = h_i;
            } else if col == 'o' {
                x2 = w_i;
                y2 = h_i;
            }
        }
    }
    println!(
        "{}",
        (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
    );
}
