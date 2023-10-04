fn main() {
    let mut x = 99;
    while x >= 1 {
        if x == 1 {
            println!("{} bottle of beer on the wall, {} bottle of beer", x, x);
        } else {
            println!("{} bottles of beer on the wall, {} bottles of beer", x, x);
        }
        x = x - 1;
        if x != 0 {
            println!(
                "Take one down and pass it around, {} bottles of beer on the wall.",
                x
            );
        } else {
            println!("Take one down and pass it around, no more bottles of beer on the wall.");
        }
    }
    println!(
        "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall"
    );
}
