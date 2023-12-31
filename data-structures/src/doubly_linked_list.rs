use std::rc::Rc; 
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

#[derive(Debug, Clone)]
struct TransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
            },
            None => {
                self.head = Some(new_node.clone());
            }
        }

        self.length += 1;
        self.tail = Some(new_node);
    }

    pub fn prepend(&mut self, value: String) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
            },
            None => {
                self.tail = Some(new_node.clone());
            }
        }

        self.length += 1;
        self.head = Some(new_node);
    }

    pub fn pop_back(&mut self) -> Option<String> {
        self.tail.take().map(|old_tail| {
            if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
            } else {
                self.head.take();
            }

            self.length -= 1;
            Rc::try_unwrap(old_tail)
                .ok()
                .unwrap()
                .into_inner()
                .value
        })
    }

    pub fn pop_front(&mut self) -> Option<String> {
        self.head.take().map(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(old_head)
                .ok()
                .unwrap()
                .into_inner()
                .value
        })
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator {
            current: start_at,
        }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}

#[test]
fn test_transaction_log() {
    let mut log = TransactionLog::new_empty();
    
    log.append("1".to_string());
    log.append("2".to_string());
    log.append("3".to_string());
    log.append("4".to_string());
    log.append("5".to_string());
    log.prepend("-1".to_string());
    log.prepend("-2".to_string());
    log.prepend("-3".to_string());
    log.prepend("-4".to_string());
    log.prepend("-5".to_string());
    let log_itr = log.iter();
    for i in log_itr {
        println!("{}", i);
    }

    log.pop_back();
    log.pop_back();
    log.pop_front();
    log.pop_front();
    let log_itr = log.iter();
    for i in log_itr {
        println!("{}", i);
    }
}
