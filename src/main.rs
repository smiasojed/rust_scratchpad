use std::fmt::{Debug, Display};
use std::cmp::PartialEq;

#[derive(PartialEq)]
#[derive(Debug)]
struct Item<T> {
    x: T,
    address: Option<Box<Item<T>>>
}

struct List<T: Display + PartialEq + Debug> {
    head: Option<Box<Item<T>>>
}

impl<T: Display + PartialEq + Debug> List<T> {
    fn push(&mut self, v: T){
        let el = Item {x: v, address: None};
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
            println!("List value {}", y.x);
            e = &y.address;
        }
    }

    fn delete_all2(&mut self, v: T) {
        let mut e = &mut self.head;
        while e.is_some() {
            let next = &mut e.as_mut().unwrap().address;
            if next.is_some() {
                if next.as_ref().unwrap().x == v {
                    let next_next = next.as_mut().unwrap().address.take();
                    e.as_mut().unwrap().address = next_next;
                }
            }
            e = &mut e.as_mut().unwrap().address;
        }
    }

    fn delete_all(&mut self, v: T) {
        let mut e = &mut self.head;
        if let Some(ref mut y) = e {
            //Why it can not be a part of loop (Writing to borrowed variable error)?
            if y.x == v {
                *e = y.address.take();
            }
        }
        while let Some(ref mut y) = e {
            /*if y.x == v {
                *e = y.address.clone();
            }*/
            if let Some(ref mut z) = &mut y.address {
                if z.x == v {
                    y.address = z.address.take();
                }
            }
            e = &mut y.address;
        }
    }
}

fn main() {
    let mut list = List::<u32> {head: None};
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(2);
    list.delete_all2(1);
    list.push(5);
    list.push(6);
    list.push(7);
    list.delete_all2(2);
    list.delete_all(7);
    list.print();
}
