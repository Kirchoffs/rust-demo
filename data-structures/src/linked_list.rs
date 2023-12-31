use std::rc::Rc;
use std::cell::RefCell;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
        }))
    }
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
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
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_node) => old_node.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        self.length += 1;
        self.tail = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .unwrap()
                .into_inner()
                .value
        })
    }
}

#[test]
fn test_transaction_log() {
    let mut log = TransactionLog::new_empty();
    log.append("hello".to_string());
    log.append("world".to_string());
    assert_eq!(log.length, 2);
    assert_eq!(log.pop(), Some("hello".to_string()));
    assert_eq!(log.pop(), Some("world".to_string()));
    assert_eq!(log.length, 0);
}
