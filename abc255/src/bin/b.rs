use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        light_man_ary: [usize; k],
        arr: [(i64, i64); n]
    }
    let mut light_man_point_ary: Vec<(i64, i64)> = vec![];
    for (index, point) in arr.iter().enumerate() {
        if light_man_ary.iter().any(|e| *e == index + 1) {
            light_man_point_ary.push(*point);
        }
    }

    let mut distance_ary = vec![std::f64::INFINITY; n];

    for (index, point) in arr.iter().enumerate() {
        for l_point in light_man_point_ary.iter() {
            let val =
                ((l_point.0 - point.0).pow(2) as f64 + (l_point.1 - point.1).pow(2) as f64).sqrt();
            if distance_ary[index] > val {
                distance_ary[index] = val;
            }
            // println!(
            //     "{:?}",
            //     ((l_point.0 - point.0).pow(2) as f64 + (l_point.1 - point.1).pow(2) as f64).sqrt()
            // );
        }
    }

    let mut max = std::f64::NEG_INFINITY;

    for distance in distance_ary {
        if max < distance {
            max = distance;
        }
    }

    println!("{}", max);
}
