fn main() {
    let x: i32 = 3;
    let y: i32 = 10;
    println!("{} {}", y, x);
    println!("{}", x + y);

    // from rust documentation!!!
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", 3, "Bob");
    // :)
    println!("{sub} {verb} {object}",
             object="the lazy dog",
             sub="the quick brown orange",
             verb="rust programming language");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10 the number:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c   
    println!("{number:>1000}", number=10);
    println!("{number:>100}", number="this is text but assign a var name number and this text is spacing 100");      
    // You can pad numbers with extra zeroes,
    println!("{number:0>10}", number=1); // 0000000001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "added");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}