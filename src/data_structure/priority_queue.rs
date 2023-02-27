use priority_queue::PriorityQueue;

pub fn test_priority_queue() {
    let mut q = PriorityQueue::new();
  
    // Add elements to an empty queue:
    assert!(q.is_empty());
    q.push("Maths", 10);
    q.push("Physics", 5);
    q.push("Chemistry", 15);
  
    // Check if Chemistry has the highest priority:
    assert_eq!(q.peek(), Some((&"Chemistry", &15)));
  
    // Give highest priority to Maths:
    q.change_priority("Maths", 20);
  
    // Check if Maths has the highest priority:
    assert_eq!(q.peek(), Some((&"Maths", &20)));
    println!("Tests passed!");
  }