// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub previous: *mut Node<T>,
    pub next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            previous: std::ptr::null_mut(), // nullptr equivelant
            next: std::ptr::null_mut(),     // nullptr equivelant
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: *mut Node<T>,
    pub tail: *mut Node<T>,
    size: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: *mut Node<T>,
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let head = self.head;
        Cursor {
            list: self,
            current: head,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let tail = self.tail;
        Cursor {
            list: self,
            current: tail,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        if self.head.is_null() {
            return Iter(None);
        }
        unsafe { Iter(Some(&(*self.head))) }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut ptr = self.cursor_front();

        while ptr.take().is_some() {}
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let ref_mut: &mut T = &mut (*self.current).value;
            Some(ref_mut)
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            panic!();
        }
        unsafe {
            self.current = (*self.current).next;
        }
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            panic!();
        }
        unsafe {
            self.current = (*self.current).previous;
        }
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.current.is_null() || self.list.size == 0 {
            return None;
        }
        let c = self.current;
        if self.list.size == 1 {
            // head, tail, current point to this adress
            self.list.head = std::ptr::null_mut();
            self.list.tail = std::ptr::null_mut();
            self.current = std::ptr::null_mut();
            self.list.size = 0;
        } else if self.current == self.list.head {
            unsafe {
                self.current = (*self.current).next;
            }
            self.list.head = self.current;
            self.list.size -= 1;
        } else if self.current == self.list.tail {
            unsafe {
                self.current = (*self.current).previous;
            }
            self.list.tail = self.current;
            self.list.size -= 1;
        } else {
            unsafe {
                let next_node = (*self.current).next;
                let prev_node = (*self.current).previous;
                (*next_node).previous = prev_node;
                (*prev_node).next = next_node;
                self.current = next_node;
            }
        }

        unsafe {
            let v = Box::from_raw(c);
            Some(v.value)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let ptr = Box::into_raw(Box::new(Node::new(element)));
        if self.current.is_null() || self.list.size == 0 {
            // Empty LinkedList
            self.list.head = ptr;
            self.list.tail = self.list.head;
            self.current = self.list.head;
        } else if self.list.size == 1 {
            unsafe {
                (*self.list.head).next = ptr;
                (*ptr).previous = self.list.head;
                self.list.tail = ptr
            }
        } else {
            if self.current == self.list.tail {
                self.list.tail = ptr; // new tail
            }
            unsafe {
                let next_node = (*self.current).next;

                // config new node
                (*ptr).previous = self.current;
                (*ptr).next = next_node;

                //config old current
                (*self.current).next = ptr;
                if !next_node.is_null() {
                    (*next_node).previous = ptr;
                }
            }
        }
        self.list.size += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let ptr = Box::into_raw(Box::new(Node::new(element)));
        if self.current.is_null() || self.list.size == 0 {
            // Empty LinkedList
            self.list.head = ptr;
            self.list.tail = self.list.head;
            self.current = self.list.head;
        } else if self.list.size == 1 {
            unsafe {
                (*self.list.head).previous = ptr;
                (*ptr).next = self.list.head;
            }
            self.list.head = ptr;
        } else {
            if self.current == self.list.head {
                self.list.head = ptr; // new head
            }
            unsafe {
                let prev_node = (*self.current).previous;

                // config new node
                (*ptr).previous = prev_node;
                (*ptr).next = self.current;

                //config old current
                (*self.current).previous = ptr;
                if !prev_node.is_null() {
                    (*prev_node).next = ptr;
                }
            }
        }
        self.list.size += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let v = &self.0?.value;
        if self.0?.next.is_null() {
            self.0 = None;
        } else {
            unsafe {
                self.0 = Some(&(*self.0?.next));
            }
        }
        Some(v)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
