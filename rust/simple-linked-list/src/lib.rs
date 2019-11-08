pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{head:None,size:0}
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node{ data:element,next:self.head.take() }));
        self.size +=1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let t = self.head.take();
        match t{
            Some(mut n) => {
                self.head = n.next.take();
                self.size -=1;
                return Some(n.data);
            },
            None => None
            
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        for _ in 0..self.size{
            result.push(self.pop().unwrap());
        }
        result
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut result:SimpleLinkedList<T> = SimpleLinkedList::new();
        for i in item{
            result.push(i.clone());
        }
        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> where T : Clone{
    fn into(mut self) -> Vec<T> {
        let mut result:Vec<T> = Vec::new();
        for _ in 0..self.size{
            result.insert(0,(self.pop().unwrap()));
        }
        result
    }
}