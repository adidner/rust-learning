// Arrays - Fixed list where elements are the same data types

pub fn run(){
    let numbers: [i32; 5] = [1,2,3,4,5]; //data type, number elements 
    // let numbers: [i32; 5] = [1,2,3,4]; // this won't work because length is not exactly 5
    println!("{:?}", numbers); 
    
    // get single val
    println!("single value: {}", numbers[0]); 
    
    
    
    let mut new_numbers: [i32; 5] = [1,2,3,4,5]; //cannot change but can reassign
    new_numbers[2] = 20;
    println!("{:?}", new_numbers); 
    
    //get length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2]; // 0 to 2
    println!("Slice: {:?}", slice); 

}