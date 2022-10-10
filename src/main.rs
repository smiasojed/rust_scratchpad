#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Item {
    x: u32,
    address: Option<Box<Item>>
}

struct List {
    head: Option<Box<Item>>
}

impl List {
    fn push(&mut self, v: u32){
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

    fn delete(&mut self, v: u32) {
        let mut e = &mut self.head;
        if let Some(ref mut y) = e {
            //Why it can not be a part of loop (Writing to borrowed variable error)?
            if y.x == v {
                *e = y.address.clone();
            }
        }
        while let Some(ref mut y) = e {
            /*if y.x == v {
                *e = y.address.clone();
            }*/
            if let Some(ref mut z) = &mut y.address {
                if z.x == v {
                    y.address = z.address.clone();
                    break;
                }
            }
            e = &mut y.address;
        }
    }

}

fn main() {
    let mut list = List {head: None};
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(2);
    list.delete(1);
    list.push(5);
    list.push(6);
    list.push(7);
    list.delete(2);
    list.delete(2);
    list.print();
}
