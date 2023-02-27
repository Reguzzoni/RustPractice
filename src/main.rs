#[path = "data_structure/priority_queue.rs"] mod priority_queue;
#[path = "data_structure/d_ary_heap.rs"] mod d_ary_heap;
#[path = "algorithm/djikstra.rs"] mod djikstra;

fn main() {
  // test priority queue
  priority_queue::test_priority_queue();

  // test dijkstra
  djikstra::test_djikstra();

  // test dary heap
  d_ary_heap::test_dary_heap();
}