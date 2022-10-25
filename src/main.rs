use std::fmt::{Debug, Display};
use std::cmp::PartialEq;

#[derive(PartialEq)]
#[derive(Debug)]
struct Item<T> {
    value: T,
    address: Option<Box<Item<T>>>
}

struct List<T: Display + PartialEq + Debug + Clone> {
    head: Option<Box<Item<T>>>
}

impl<T: Display + PartialEq + Debug + Clone> List<T> {
    fn new() -> List<T> {
        List {
            head: None
        }
    }

    fn push(&mut self, v: T) {
        let el = Item {value: v, address: None};
        let mut e = &mut self.head;
        loop {
            if let Some(ref mut y) = e { 
                e = &mut y.address;
            } else {
                break;
            }
        }
        *e = Some(Box::new(el));
    }

    fn print(&self) {
        let mut e = &self.head;
        while let Some(y) = e {
            println!("List value {}", y.value);
            e = &y.address;
        }
    }

    fn delete_all(&mut self, v: T) {
        let mut e = &mut self.head;
        if let Some(ref mut y) = e {
            //Why it can not be a part of loop (Writing to borrowed variable error)?
            if y.value == v {
                *e = y.address.take();
            }
        }
        while let Some(ref mut y) = e {
            /*if y.value == v {
                *e = y.address.clone();
            }*/
            if let Some(ref mut z) = &mut y.address {
                if z.value == v {
                    y.address = z.address.take();
                }
            }
            e = &mut y.address;
        }
    }

    fn delete_all2(&mut self, v: T) {
        let mut e = &mut self.head;
        if e.is_some() && e.as_ref().unwrap().value == v {
            *e = e.as_mut().unwrap().address.take();
        }
        while e.is_some() {
            let next = &mut e.as_mut().unwrap().address;
            if next.is_some() {
                if next.as_ref().unwrap().value == v {
                    let next_next = next.as_mut().unwrap().address.take();
                    e.as_mut().unwrap().address = next_next;
                }
            }
            e = &mut e.as_mut().unwrap().address;
        }
    }

    fn dump_to_vec(&self) -> Vec<T> {
        let mut e = &self.head;
        let mut vec = Vec::new();
        while let Some(y) = e {
            vec.push(y.value.clone());
            e = &y.address;
        }
        vec
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(2);
    list.delete_all(2);
    list.push(5);
    list.push(6);
    list.push(7);
    list.delete_all2(1);
    let v = list.dump_to_vec();
    println!("{:?}", v);
    list.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_list() {
        let mut list = List::new();
        let range = 1..=10;
        for x in range.clone() {
            list.push(x);
        }
        let vec = range.collect::<Vec<_>>();
        assert_eq!(list.dump_to_vec(), vec);
    }

    #[test]
    fn delete_list_item() {
        let mut list = List::new();
        let range = 1..=10;
        for x in range.clone() {
            list.push(x);
        }
        list.delete_all(2);
        let vec = range.filter(|x| *x!=2).collect::<Vec<_>>();
        assert_eq!(list.dump_to_vec(), vec);
    }

    #[test]
    fn delete2_list_item() {
        let mut list = List::new();
        let range = 1..=10;
        for x in range.clone() {
            list.push(x);
        }
        list.delete_all(1);
        let vec = range.filter(|x| *x!=1).collect::<Vec<_>>();
        assert_eq!(list.dump_to_vec(), vec);
    }
}