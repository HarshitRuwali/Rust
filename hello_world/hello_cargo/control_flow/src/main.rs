fn main() {
    let x = 32;

    if x == 32{
    	equal(x);
    }
    else{
    	not_equal();
    }

}

fn equal(x : i32){
	println!("The vaule of x is {}", x);
}

fn not_equal()
{
	print!("The value of x is not 32");
}