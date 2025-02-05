use std::collections::HashMap;


fn main() {
    //Vector is the most used 
    //Works like a stack 
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(-1);
    let last_value = v.pop();
    println!("Last value was {:?}",last_value);
    println!("First value was {}",v[0]);

    //Init vector
    let mut vec = vec![2,3,4];
    println!("First value of vec was {}",vec[0]);

    //HashMaps
    let mut h: HashMap<u8,bool> = HashMap::new();
    h.insert(10,true);
    h.insert(6,false);
    let have_ten = h.remove(&10);
    let have_six_unwrapped = h.remove(&6).unwrap(); //Returns option so we need the unwrap 
    println!("First value of h was {:?}", have_ten);
    println!("Second value of h was {:?}", have_six_unwrapped);
}

//VecDeque 
//Hashset
//LinkedList
//BinaryHeap
//BTTreeMap
//BTreeSet