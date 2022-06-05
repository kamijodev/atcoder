use proconio::input;

fn main() {
    input! {
        n: String
    }
    println!("{}", &n[(n.len() - 2)..]);
}
