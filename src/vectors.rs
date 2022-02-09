
// Vectors - Resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5]; //data type, number elements    
    
    numbers[2] = 20;//re-assign

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value
    numbers.pop();
    
    println!("{:?}", numbers); 
    println!("single val: {}", numbers[0]); 
    
    //get length
    println!("Vector Length: {}", numbers.len());
    
    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    
    // Get Slice
    let slice: &[i32] = &numbers[0..2]; // 0 to 2
    println!("Slice: {:?}", slice); 
    
    
    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}",x)
    }
    
    // Loop and Mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    
    
    println!("Numbers Vec: {:?}", numbers); 

}