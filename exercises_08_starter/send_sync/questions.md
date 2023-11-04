1) I saw someone's code fail to compile because they 
were trying to send non-thread-safe data across threads. 
How does the Rust language allow for static (i.e. at compile time)
guarantees that specific data can be sent/shared acrosss threads?

1. Ownership and borrowing: Rust's ownership model ensures that only one thread can have mutable access to a piece of data at a time. This prevents data races, which are a common source of concurrency errors.
2. The Send and Sync traits: The Send trait indicates that a type can be safely sent between threads. 
The Sync trait indicates that a type can be safely shared between threads.
3. Compile-time checks: The Rust compiler performs static checks to ensure that data is only sent/shared across threads in a safe manner.
To send/share data across threads, it must first be wrapped in a type that implements the Send and Sync traits.
If the compiler detects that data is being sent/shared across threads in a way that is not safe, it will generate an error.

2) Do you have to then implement the Send and Sync traits for 
every piece of data (i.e. a struct) you want to share and send across threads?

No. Some common types that implement Send and Sync include:
  Primitive types, such as i32, f64, and bool
  Immutable references to data
  Atomic types, such as AtomicU32 and AtomicBool
  Data structures that are designed to be thread-safe, such as Arc<T> and Mutex<T>


3) What types in the course have I seen that aren't Send? Give one example, 
and explain why that type isn't Send 

 Rc<RefCell<T>> is a non-thread-safe type because it allows shared ownership and mutable access to the data, and there are no guarantees that multiple threads won't try to mutate the data at the same time, potentially leading to data races and safety issues. Therefore, Rc<RefCell<T>> does not implement the Send trait, as it cannot be safely transferred between threads.


4) What is the relationship between Send and Sync? Does this relate
to Rust's Ownership system somehow?

Send means that it is safe to move a value of the type across thread boundaries. This means that the value can be sent from one thread to another.
Sync means that it is safe to share a reference to the type across thread boundaries. This means that the reference can be passed to multiple threads and used concurrently.
The relationship between Send and Sync is that Sync implies Send. This means that if a type is Sync, then it is also safe to move a value of the type across thread boundaries.
The Send and Sync traits are related to Rust's ownership system in that they are used to enforce safety when using concurrency. 


5) Are there any types that could be Send but NOT Sync? Is that even possible?
Yes.
One example is the Cell struct. A Cell is a wrapper around a mutable value that allows you to modify the value without using a mutex. This is useful for cases where you need to modify a shared value from multiple threads, but you do not need to synchronize the access.



6) Could we implement Send ourselves using safe rust? why/why not?
Yes.


Yes, it is possible to implement the Send trait yourself using safe Rust. However, this is a complex task and should only be attempted by experienced programmers.

To implement Send safely, you must ensure that your type satisfies the following conditions:

It must not contain any references to other types that are not Send.
It must not contain any mutable state that is not thread-safe.
This can be difficult to achieve, especially for types that contain complex data structures or interior mutability.

If you are unsure whether or not you can safely implement Send for your type, it is best to err on the side of caution and not implement it at all.

Here are some reasons why you might want to implement Send yourself:

To make your type more reusable and compatible with other libraries.
To improve the performance of your code by avoiding the overhead of using a mutex.
Here are some reasons why you might not want to implement Send yourself:

It is a complex task that is easy to get wrong.
If you implement Send incorrectly, you can introduce data races and other concurrency errors into your code.
If your type is not actually thread-safe, implementing Send will give the compiler a false sense of security and could lead to unexpected errors.
Overall, whether or not you should implement Send for your type is a decision that should be made on a case-by-case basis. If you are unsure whether or not it is safe to implement Send, it is best to err on the side of caution and not implement it at all.

Here are some tips for implementing Send safely:

Carefully consider all of the ways in which your type can be used.
Identify any potential sources of data races.
Use mutexes and other synchronization primitives to protect shared data.
Test your code thoroughly to ensure that it is thread-safe.
