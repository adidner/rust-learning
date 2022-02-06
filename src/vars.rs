pub fn run() {
    let name = "Brad"; // vars are immutable my default
    let mut age = 37; // making it mutable
    
    println!("My name is {} and I am {}", name, age);

    age = 38;
    println!("My name is {} and I am {}", name, age);


    // const

    const ID: i32 = 001; // const need defined type
    println!("ID: {}", ID);


    // assign multiple vars
    let ( my_name, my_age ) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);

}