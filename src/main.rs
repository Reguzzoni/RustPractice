use priority_queue::PriorityQueue;

use std::{
  cmp::Ordering,
  collections::{BinaryHeap, HashMap, HashSet},
};

fn main() {
  // test priority queue
  priority_queue();

  // test dijkstra
  test_djikstra();
}

fn test_djikstra() {
  let s = Vertex::new("s");
  let t = Vertex::new("t");
  let x = Vertex::new("x");
  let y = Vertex::new("y");
  let z = Vertex::new("z");

  let mut adjacency_list = HashMap::new();
  adjacency_list.insert(s, vec![(t, 10), (y, 5)]);
  adjacency_list.insert(t, vec![(y, 2), (x, 1)]);
  adjacency_list.insert(x, vec![(z, 4)]);
  adjacency_list.insert(y, vec![(t, 3), (x, 9), (z, 2)]);
  adjacency_list.insert(z, vec![(s, 7), (x, 6)]);

  let distances = dijkstra(s, &adjacency_list);

  for (v, d) in &distances {
      println!("name: {}, distance: {}", v.name, d);
  }

  assert_eq!(distances.get(&t), Some(&8));
  assert_eq!(distances.get(&s), Some(&0));
  assert_eq!(distances.get(&y), Some(&5));
  assert_eq!(distances.get(&x), Some(&9));
  assert_eq!(distances.get(&z), Some(&7));
}

fn priority_queue() {
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

fn dijkstra<'a>(
  start: Vertex<'a>,
  adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>,
) -> HashMap<Vertex<'a>, usize> {
  let mut distances = HashMap::new();
  let mut visited = HashSet::new();
  let mut to_visit = BinaryHeap::new();

  distances.insert(start, 0);
  to_visit.push(Visit {
      vertex: start,
      distance: 0,
  });

  while let Some(Visit { vertex, distance }) = to_visit.pop() {
      if !visited.insert(vertex) {
          // Already visited this node
          continue;
      }

      if let Some(neighbors) = adjacency_list.get(&vertex) {
          for (neighbor, cost) in neighbors {
              let new_distance = distance + cost;
              let is_shorter = distances
                  .get(&neighbor)
                  .map_or(true, |&current| new_distance < current);

              if is_shorter {
                  distances.insert(*neighbor, new_distance);
                  to_visit.push(Visit {
                      vertex: *neighbor,
                      distance: new_distance,
                  });
              }
          }
      }
  }

  distances
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex<'a> {
  name: &'a str,
}

impl<'a> Vertex<'a> {
  fn new(name: &'a str) -> Vertex<'a> {
      Vertex { name }
  }
}

#[derive(Debug)]
struct Visit<V> {
  vertex: V,
  distance: usize,
}

impl<V> Ord for Visit<V> {
  fn cmp(&self, other: &Self) -> Ordering {
      other.distance.cmp(&self.distance)
  }
}

impl<V> PartialOrd for Visit<V> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl<V> PartialEq for Visit<V> {
  fn eq(&self, other: &Self) -> bool {
      self.distance.eq(&other.distance)
  }
}

impl<V> Eq for Visit<V> {}