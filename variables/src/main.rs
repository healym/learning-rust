fn main() {
    let x = 5;
    //x = 6; // would create a compile error, x isn't mutable
    println!("x: {}", x);
    let x = x + 1; // is fine, via variable shadowing
    let x = x * 2;
    println!("x: {}", x);

    let mut y = 10;

    println!("y: {}");
    y = 15;
    println!("since y is mutable, y = {}", y);

    /* Type changing with variable shadowing */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{} spaces", spaces);

    /* this would cause an error, since variables cannot change type
     * let mut test = 15;
     * test = "string";
     */
}
