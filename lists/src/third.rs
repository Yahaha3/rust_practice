// 多引用单向链表

use std::rc::Rc;

pub struct List<T> {
    data: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { data: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            data: Some(Rc::new(
                Node{
                    element: elem,
                    next: self.data.clone(),
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {data: self.data.as_ref().and_then(|node| node.next.clone())}
    }

    pub fn head(&self) ->Option<&T> {
        self.data.as_deref().map(|node| &node.element)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {next: self.data.as_deref()}
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

// pub struct IntoIter<T>(List<T>);

// impl<T> List<T> {
//     pub fn into_iter(self) -> IntoIter<T> {
//         IntoIter(self)
//     }
// }

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.head().map(|node| node.clone())
//     }
// }

// pub struct IterMut<'a, T> {
//     next: Option<&'a mut Node<T>>,
// }

// impl <T> List<T> {
//     pub fn iter_mut(&mut self) -> IterMut<'_, T> {
//         IterMut { next: self.data.as_deref_mut() }
//     }
// }

// impl <'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|node| {
//             self.next = node.next.as_deref_mut();
//             &mut node.element
//         })
//     }
    
// }


#[cfg(test)]
mod test {
    use super::List;

    
    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter(){
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}