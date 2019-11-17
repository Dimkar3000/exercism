# Doubly Linked List
This a very common data struct that you encounter as a developer with a lot of use cases. It is also relatively easy to implement. I implement it in rust for an [exercism](https://exercism.io/) exercise. This article provides a top level walkthrough of the exercise without any implementation details that will spoil the solution, but I do provide a solution at the end.  

# Explanation 
Most of the problem is defining the proper struct to reference and store your data. The basic block of a `linked list` is a **Node** that stores the data and 2 pointers (previous and next). For me it looks like this 

```rust 
pub struct Node<T> {
    pub value: T,
    pub previous: *mut Node<T>,
    pub next: *mut Node<T>,
}
```

Now the `doubly linked list` needs to store a referense to the first and also to the last Node. We access the nodes in the middle by ***walking*** the list from top to bottom and reverse. Here is the struct:

```rust
pub struct LinkedList<T> {
    pub head: *mut Node<T>,
    pub tail: *mut Node<T>,
    size: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: *mut Node<T>,
}
```

A **Cursor**, as the name suggests, points to specific element in the list and allows us to edit the list, add/remove nodes. It is also important to 'point out' that a cursor can change the head or the tail of the list and that's why he needs a mutable reference to it.   

# Unsafe
You can clearly see in the code above that I store pointer inside my structs. This is a key difference of this exercise. You **have** to use unsafe in order to dereference this pointer and that is ok. The point of this exercise is to use unsafe code in order to provide a safe API for the users. It is your job to ensure that your code never fails unexpectedly. 

# More
Now you are ready to solve this exercise on your own. If you need any more help, I will provide links to resources that can help you and of course my own solution. I suggest you try solving it first.
- [Geeksforgeeks](https://www.geeksforgeeks.org/doubly-linked-list/)
- [Wikipedia](https://en.wikipedia.org/wiki/Doubly_linked_list)
- [My Solution](https://github.com/Dimkar3000/exercism/tree/master/rust/doubly-linked-list)