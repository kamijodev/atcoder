use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    let mut arr = [a, b, c];
    arr.sort();
    let ans = if arr[1] == b { "Yes" } else { "No" };
    println!("{}", ans);
}
