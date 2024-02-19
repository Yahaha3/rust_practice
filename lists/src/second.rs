// 单引用单向链表
pub struct List<T> {
    data: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}


// node0 <- node1 <- node2 <- node3
impl<T> List<T> {
    pub fn new() -> Self {
        List {data: None}
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            element: elem,
            next: self.data.take(),
        });

        self.data = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.take() {
            None => None,
            Some(node) => {
                self.data = node.next;
                Some(node.element)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.as_ref().map(|node| {
            &node.element
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut().map(|node|{
            &mut node.element
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.data.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {next: self.data.as_deref()}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl <T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.data.as_deref_mut() }
    }
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
    
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));  
        assert_eq!(list.pop(), None);

    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push('A');
        list.push('B');
        list.push('C');

        assert_eq!(list.peek(), Some(&'C'));
        assert_eq!(list.peek_mut(), Some(&mut 'C'));

        list.peek_mut().map(|value| {
            *value = 'D'
        });

        assert_eq!(list.peek(), Some(&'D'));
        assert_eq!(list.pop(), Some('D'));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);list.push(3);list.push(2);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);list.push(3);list.push(2);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);list.push(3);list.push(2);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}