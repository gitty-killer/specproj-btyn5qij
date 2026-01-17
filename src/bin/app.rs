use '$name'::score_all;

fn main() {
    let out = score_all(&[1, 2, 3]);
    println!("{:?}", out);
}
