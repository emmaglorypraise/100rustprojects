use std::fmt;
 
// Node struct representing each element
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
 
// Linked list with a head pointer
struct LinkedList {
    head: Option<Box<Node>>,
}
 
impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
 
    fn push_front(&mut self, val: i32) {
        let new_node = Box::new(Node {
            value: val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
 
    fn push_back(&mut self, val: i32) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node { value: val, next: None }));
    }

    fn delete(&mut self, val: i32) -> bool {
        let mut current = &mut self.head;
        loop {
            match current {
                None => return false,
                Some(node) if node.value == val => {
                    *current = node.next.take();
                    return true;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }
 
    fn contains(&self, val: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == val {
                return true;
            }
            current = &node.next;
        }
        false
    }
 
    fn print(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}
 
// Optional display trait for prettier output
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "{} -> ", node.value)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}
 
fn main() {
    let mut list = LinkedList::new();
    list.push_front(10);
    list.push_back(20);
    list.push_back(30);
    list.print(); // List: 10 -> 20 -> 30 -> None
 
    println!("Contains 20? {}", list.contains(20));
    println!("Deleting 20...");
    list.delete(20);
    list.print(); // List: 10 -> 30 -> None
}