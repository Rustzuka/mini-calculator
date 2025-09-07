fn main() {
    let (sum, product, difference, quotient):(i32, i32, i32, i32)= calculator(20,5);
    println!("Sum of 20 and 5 is {}", sum);
    println!("Product of 20 and 5 is {}", product);
    println!("Difference between 20 and 5 is {}", difference);
    println!("Quotient of 20 and 5 is {}", quotient);
}

fn calculator(a:i32, b:i32)->(i32, i32, i32, i32){
    let sum = a+b ;
    let product = a*b ;
    let difference = a-b ;
    let quotient = a/b ;
    (sum, product, difference, quotient)
}
