/*fn main2() {
    let mut x = 3;
    let mut y = 6;
    let eq_to_x = |z| z == x;
    // x = 4;
    println!("res {} and y {}", eq_to_x(3), y);
}*/

fn main() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|&x| x + 1).collect();
    println!("v1: {:?} ; v2: {:?}", v1, v2);
}
