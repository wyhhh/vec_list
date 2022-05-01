# Rust Vec Doubly Linked List
Just like doubly linked list(e.g. std::LinkedList), but supports that returning a index of the vec when push. And you can remove an element by a valid index.
Enhanced Edition of slab crate.

# Methods
- push_back()/push_front()
- get()/get_mut()/\[]/\[mut]
- front()/front_mut()/back()/back_mut()
- pop_front()/pop_back()

All operations above are O(1) worst time complexity, except `pushs` are average O(1).

You can also use some like as a `Map` which the key is always usize.