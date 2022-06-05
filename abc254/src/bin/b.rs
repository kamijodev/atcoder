use proconio::input;

fn main() {
    input! {
        n: i32
    }
    let mut v = vec![];
    for i in 0..n as usize {
        let v2: Vec<i32> = Vec::new();
        v.push(v2);
        for j in 0..(i + 1) as usize {
            if j == 0 || j == i {
                v[i].push(1);
            } else {
                let num = v[i - 1][j - 1] + v[i - 1][j];
                v[i].push(num);
            }
        }
        let row: Vec<String> = v[i].iter().map(|&x| x.to_string()).collect();
        println!("{}", row.join(" "));
    }
}
