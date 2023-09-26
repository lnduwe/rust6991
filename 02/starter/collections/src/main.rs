use std::collections::{hash_map, HashMap, LinkedList, VecDeque};

const MAX_ITER: i32 = 900000;

fn main() {
    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    // TODO: your code here, for linked list insertions
    linked_list_operations();
    // TODO: your code here, for hashmap insertions
    hashmap_operations();
    // TODO: your text explanation to the questions in the spec

    
}

// Which collection type was the fastest for adding and removing elements? Why do you think this was the case?
//   VecDeque was the fastest way. It is a double-ended queue, the time complexity for insertion and deletion operations is O(1) therefore it is really fast..

// When would you consider using VecDeque over Vec?
//   If we need quick random access to certain items, I would use Vec. If we need to perform efficient insertion and deletion operations at both ends, I prefer VecDeque. 

// When would you consider using LinkedList over Vec?
//   If there is a need for frequent insertion and deletion operations, I would consider using LinkedList. 

// Did the results suprise you? Why or why not?.
//   No.  The time complexity of both vec and hashmap is O(n) which is obviously much slower. 
//   LinkedList is also good at insertion/deletion but it has more complex data structure, each element points to the previous and next elements , so its unsatisfactory scores can be expected.



/// measure the insertion and removal
/// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

/// measure the insertion and removal
/// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn linked_list_operations() {
    let mut linkedlist: LinkedList<i32> = LinkedList::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        linkedlist.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== LinkedList ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        linkedlist.pop_front();
    }
    let time_end = std::time::Instant::now();
    println!("remove: {:?}", time_end - time_start);
}

fn hashmap_operations() {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hm.insert(i, i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Hashmap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();

    for i in 0..MAX_ITER {
        hm.remove(&i);
    }
    let time_end = std::time::Instant::now();
    println!("remove: {:?}", time_end - time_start);
}
