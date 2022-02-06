// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let hello = "Hello"; // primitive
    let mut mutible = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());
    println!("Length: {}", mutible.len());

    mutible.push('W'); // for chars

    mutible.push_str("orld"); // for strings


    // capacity in bytes
    println!("Capcity: {}", mutible.capacity());

    //check if empty
    println!("Is Exmpty: {}", mutible.is_empty());

    // contains
    println!("Contains 'World' {}", mutible.contains("World"));

    // replace
    println!("Replace: {}", mutible.replace("World", "There"));

    //Loop through string by white space
    for word in mutible.split_whitespace() {
        println!("{}", word)
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion Testing -> helpful for debugging
    assert_eq!(2, s.len()); // when passes nothing in output, only throws error when fails
    assert_eq!(10, s.capacity());

    // println!("hello var: {}", hello);
    // println!("mutible var: {}", mutible);
}