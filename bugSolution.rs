fn main() {
    let mut x = 5;
    { //This is solution, use scope to control the lifetime of mutable references
        let y = &mut x; 
        *y = 6;
        println!("x = {}", x);
    }
    { // Create a new scope for the second mutable borrow
        let z = &mut x;
        *z = 7;
        println!("x = {}", x);
    }
    println!("x = {}", x);
} 