mod print; //name of file
mod vars; 
mod data_types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

fn main(){
    print::run();
    vars::run();
    data_types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();

}



// //comments

// // fn main() {
// //     println!("Hello, world!");
// //     println!("Hello, world! {}", 31);
// //     println!("Hello, world! {0} {1}", "alice", "bob");
// //     println!("Hello, world! {object} {person} {1}", object="soemthing", person="tad strange");
// //     println!("Hello, world! {:b}", 1);
// //     println!("{number:>width$}", number=1, width=6);
// // }


// // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// // is a structure which contains a single `i32`.
// #[derive(Debug)]
// struct Structure(i32);

// // Put a `Structure` inside of the structure `Deep`. Make it printable
// // also.
// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//     // Printing with `{:?}` is similar to with `{}`.
//     println!("{:?} months in a year.", 12);
//     println!("{1:?} {0:?} is the {actor:?} name.",
//              "Slater",
//              "Christian",
//              actor="actor's");

//     // `Structure` is printable!
//     println!("Now {:?} will print!", Structure(3));
    
//     // The problem with `derive` is there is no control over how
//     // the results look. What if I want this to just show a `7`?
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }
