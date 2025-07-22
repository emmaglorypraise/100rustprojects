# Project 020 – Linked List Implementation

## What I Built
A Rust CLI that implements a singly linked list from scratch in Rust using Box and Option. 

## What I Learned
- Boxes(`Box<T>`) allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. It is a smart pointer.

- A linked list is a linear data structure that stores data in nodes, where each node contains a data element and a reference (or link) to the next node in the sequence. This structure allows for efficient insertion and deletion of elements, especially compared to static arrays. 

- Practiced managing heap-allocated memory, manual traversal, insertion, deletion, and value search—without relying on built-in containers like Vec.

## Notes

When to use Box:
- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

Example:
```
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```








