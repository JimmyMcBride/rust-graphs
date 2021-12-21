/// List of items. Last in, first out...
pub struct Stack<T> {
	stack: Vec<T>,
}

impl<T> Stack<T> {
	/// Creates a new and empty stack...
	pub fn new() -> Self {
		Stack { stack: Vec::new() }
	}

	/// Gets the length of the stack...
	pub fn len(&self) -> usize {
		self.stack.len()
	}

	/// Pops the end value off the stack...
	pub fn pop(&mut self) -> Option<T> {
		self.stack.pop()
	}

	/// Appends an item to the end of the stack...
	pub fn append(
		&mut self,
		item: T,
	) {
		self.stack.push(item)
	}

	/// Checks if the stack is empty or not...
	pub fn is_empty(&self) -> bool {
		self.stack.is_empty()
	}

	/// Peeks at the last item in the stack...
	pub fn peek(&self) -> Option<&T> {
		self.stack.last()
	}
}
