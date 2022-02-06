pub fn run(){
    //Print to console
    println!("Hello, print.rs file!");

    //basic
    println!("{}", 1);
    println!("{} me {}", 1, 2);
    
    //positional
    println!("{0} me {1} from {0}", "person", "elephant");
    
    //named
    println!("{personType} me {activity} ", personType = "person", activity = "elephant");
    
    
    //placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //place holder for debug trait
    println!("{:?}", (12, true, "hello"))
}