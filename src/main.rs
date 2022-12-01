
#[derive(Debug, Clone)]
struct Node<T: Clone> {
  next: Option<Box<Node<T>>>,
  data: T
}

// temporary Debug concept restriction
impl<T: Clone + std::fmt::Debug> Node<T> {

  fn new(data: T) -> Self {
    Node { next: None, data }
  }

  fn insert(&mut self, ndata: T) {
    // move out of self and into a different thing
    let mut new = Node::new(ndata);
    new.next = Some(Box::new(self.clone()));
    *self = new;
  }

  fn pop(&mut self) {
    if let Some(next) = self.next.take() {
      *self = *next;
    }
  }

  fn get(&self, index: usize) -> Option<T> {
    let mut temp = self;
    let mut i = 0;
    
    while let Some(_) = temp.next {
      if i == index {
        break;
      }
      
      temp = temp.next.as_deref().unwrap();
      
      i += 1;
    }

    // is the last element chosen but an invalid index was given?
    if temp.next.is_none() && i != index {
      return None;
    }
    
    Some(temp.data.clone()) // clone self data
  }

  fn get_ref(&self, index: usize) -> Option<&T> {
    let mut temp = self;
    let mut i = 0;
    
    while let Some(_) = temp.next {
      if i == index {
        break;
      }
      
      temp = temp.next.as_deref().unwrap();
      
      i += 1;
    }

    // is the last element chosen but an invalid index was given?
    if temp.next.is_none() && i != index {
      return None;
    }
    
    Some(&temp.data)
  }

  fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    let mut temp = self;
    let mut i = 0;

    // invariant: temp.next is never None
    while let Some(_) = temp.next {
      if i == index {
        break;
      }
      
      temp = temp.next.as_deref_mut().unwrap(); // we know it is not None from the invariant
      
      i += 1;
    }

    // is the last element chosen but an invalid index was given?
    if temp.next.is_none() && i != index {
      return None;
    }
    
    Some(&mut temp.data)
  }
 
  fn length(&self) -> usize {
    let mut temp = self;
    let mut i = 1; // always atleast 1
    
    while let Some(_) = temp.next {
      temp = temp.next.as_deref().unwrap();
      i += 1;
    }
    
    i
  }

  fn map(&mut self, f: impl Fn(&T) -> T) -> &mut Self {
    let mut temp: *mut Node<_> = self as *const _ as *mut _; // mutable reference into mutable pointer of self; deref
    
    unsafe {
      while let Some(_) = (*temp).next {
        (*temp).data = f(&(*temp).data);
        temp = (*temp).next.as_deref_mut().unwrap() as *const _ as *mut _;
      }
      (*temp).data = f(&(*temp).data); // apply to last element (we know is not None, invariant)
    }
    
    self
  }

  fn filter(&mut self, p: impl Fn(&T) -> bool) -> &mut Self {
    // function to check predicate and remove nodes that do not satisfy it
    // keep reference to previous node; if the last node violates the predicate,
    // set the previous node's next pointer equal to None
    unimplemented!();
  } 
}

fn main() {
  let mut list = Node::new(1);
  list.insert(2);
  list.insert(3);
  list.insert(5);
  list.pop();

  println!("Length: {}", list.length());
  println!("0th element: {:?}", list.get(0));
  println!("Element after change: {:?}", { *list.get_mut(0).unwrap() = 6; list.get(0) });
  println!("Resulting list: {:?}", list.clone());
  println!("After maps: {:?}", list.map(|x| x + 2).map(|x| x * 2));
}

