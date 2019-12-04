fn main() {
    passing_integer(32, 64);
    let z = five();
    println!("The value of z is {}", z);
}

fn passing_integer(x : i32 , y: i32)
{
	println!("The value of x is {}", x);
	println!("The vaule of y is {}", y);
}

fn five() -> i32 {
	5
}