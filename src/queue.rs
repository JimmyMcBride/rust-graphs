/// List of items. First in, fist out...
pub struct Queue<T> {
  queue: Vec<T>,
}

impl<T> Queue<T> {
  /// Creates a new and empty queue...
  pub fn new() -> Self {
    Queue { queue: Vec::new() }
  }

  /// Gets the length of the queue...
  pub fn len(&self) -> usize {
    self.queue.len()
  }

  /// Removes the first item off the queue...
  pub fn dequeue(&mut self) -> T {
    self.queue.remove(0)
  }

  /// Appends an item to the end of the queue...
  pub fn enqueue(&mut self, item: T) {
    self.queue.push(item)
  }

  /// Checks if the queue is empty or not...
  pub fn is_empty(&self) -> bool {
    self.queue.is_empty()
  }

  /// Peeks at the last item in the queue...
  pub fn peek(&self) -> Option<&T> {
    self.queue.first()
  }
}
