fn main() {
    //RULES
    // - Each value has an owner - there is not value in memory or data that does not have a variable that owner it
    // - Only one owner 
    // - When the owner goes out of scope the value is dropped instantly
    
    /*
    The following code cause an error because the s2 = s1
    is not a copy but a move, we have transferred the ownership of the portion of heap 
    where abc is stored to s2

    let s1 = String::from("abc");
    let s2 = s1; 
    println!("{}", s1); //ERROR
     
    To not transfer the ownership we have to clone the variable like the following code
    NOTE this does not only make a copy on the stack but also in the heap (in the heap now there are two abc string allocated)
    this follows the second rule each value has one and only one owner
    */
    let s1 = String::from("abc");
    let s2 = s1.clone();
    println!("{}",s1);

    /*
    The following code will result in the same error has we are giving the same portion of heap ownership
    to the variable in the do_stuff
    
    let s3 = String::from("abc");
    do_stuff(s3);
    println!("{}",s3);

    Following two possible solution of the problem 
    */

    /* 1. Solution give back ownership - Usually not the path you want! 
     why not the path you want => usually you want the method to use the variable 
     just to do something without modifying the value 
     If the value of s is being modified this is a valid method else use ref & borrowing 
    */
    let mut s4 = String::from("String4");
    s4 = do_stuff_give_back(s4);
    println!("{}",s4);


    // Lifetime RULE
    // Lifetimes -> Rule the references must always be valid
    // The compiler will not allow you to have possible references 
    // that point to not valid values and you cannot be null

    /* 
     2. Solution Reference & Borrowing
     We pass a reference &s5 and do_stuff_ref borrows the reference of the value
     So when do_stuff_ref has finished only the reference and the borrowed ref is dropped 
     not the actual variable
    */
    let s5 = String::from("String5");
    do_stuff_ref(&s5);
    println!("{}",s5);

    /*
     2.1 Reference are immutable even if the value is mutable
     but if also the reference is mutable the we can use it to make change to the variable
    */
    let mut s6 = String::from("John");
    do_stuff_mut_ref(&mut s6); //Mutable ref
    println!("{}",s6);

    /*
    * RULE 
    * At any given time you can have either exactly one mutable reference 
    * or any number of immutable reference
    */

}

fn do_stuff(s: String){
    println!("{}",s);
}

fn do_stuff_give_back(s: String) -> String{
    println!("{}",s);
    s
}

fn do_stuff_ref(s: &String){
    println!("{}",s);
}

fn do_stuff_mut_ref(s: &mut String){
    /*
     * The dot can dereference automatically meaning 
     * that we do not care if the s is a String or a &String
     */
    s.insert_str(0,"Hi, ");
    // This would be manual dereference - get access to the value 
    (*s).insert_str(0," ");

    // NOTE
    // We have to use manual dereference for replacement 
    *s = String::from("Hi from John!"); 
}
