use rand::{
    distributions::{Distribution, Uniform,Alphanumeric},
    Rng,
};
use rand_distr::Normal;
fn main() {
    let mut rng = rand::thread_rng();
    let rng2: i32 = rand::random();
    println!("{}",rng2);
    let rng2: i32 = rand::random();
    println!("{}",rng2);
    println!("{}",rng2);
    println!("{}",rng2);
    let rng2: i32 = rand::random();
    let rng2: i32 = rand::random();
    println!("{}",rng2);
    println!("{}",rng2);
    let rng2: i32 = rand::random();
    println!("{}",rng2);
    println!("{}",rng2);
    let n1 = rng.gen::<u8>();
    println!("{}",rng2);
    let n2: u16 = rng.gen();
    let n1 = 2;

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    println!("Integer: {}", rng.gen_range(0..10));

    println!("Float: {}", rng.gen_range(0.0..10.0));

    let die = rand::distributions::Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    let mut rng3 = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng3);
    let ads :[i32;3] = [333,33,3];
        println!("{} is from a N(2, 9) distribution", v);



        let rand_tuple = rng.gen::<(i32, bool, f64)>();
        let rand_point: Point = rng.gen();
        println!("Random tuple: {:?}", rand_tuple);
        println!("Random Point: {:?}", rand_point);
    

        let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);

}
use rand::distributions::Standard;
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}
