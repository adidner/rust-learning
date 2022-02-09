// Conditional - used to check the condition ofsomething and act on it

pub fn run(){
    let age: u8 = 25;
    let check_id: bool = false;
    let knows_person_of_age = false;
    
    
    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?")
    }
    else if age < 21 && check_id {
        println!("Bartender: Sorry Baby")
        
    }
    else {
        println!("Bartender: I'll need to see your ID")
    }


    // shorthand if
    let is_of_age = if age>=21 { true } else { false };
    println!("Is of Age: {}", is_of_age);
}