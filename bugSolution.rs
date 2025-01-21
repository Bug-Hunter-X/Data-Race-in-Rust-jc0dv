fn main() {
    let mut x = 5;
    { //Creating a scope
        let y = &mut x; 
        *y = 6; 
    }
    { //Creating a scope 
        let z = &mut x; 
        *z = 7; 
    }
    println!("{}", x);
}