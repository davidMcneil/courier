use std::fmt;
use std::fmt::Debug;
use std::sync::{Arc, RwLock, Weak};

type Shared<T> = Arc<RwLock<T>>;

type WeakShared<T> = Weak<RwLock<T>>;

struct Element<T> {
    value: T,
    next: Node<T>,
}

impl<T> Element<T> {
    fn new(value: T) -> Shared<Self> {
        Arc::new(RwLock::new(Element {
            value: value,
            next: None,
        }))
    }
}

impl<T: Debug> Debug for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

type Link<T> = Shared<Element<T>>;

type Pointer<T> = WeakShared<Element<T>>;

type Node<T> = Option<Link<T>>;


pub struct Index<T> {
    index: Pointer<T>,
}

impl<T> Index<T> {
    pub fn new(cursor: &Cursor<T>) -> Index<T> {
        cursor.new_index()
    }
}

impl<T: Clone> Index<T> {
    pub fn get_copy(&self) -> Option<T> {
        self.index
            .upgrade()
            .map(|i| i.read().unwrap().value.clone())
    }
}

impl<T: Clone + Debug> Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?}]",
            self.index
                .upgrade()
                .as_ref()
                .map(|c| c.read().unwrap().value.clone())
        )
    }
}

pub struct Cursor<T> {
    cursor: Pointer<T>,
    to_head: Link<T>,
}

impl<T> Cursor<T> {
    pub fn new_head(log: &CommitLog<T>) -> Cursor<T> {
        log.new_head_cursor()
    }

    pub fn new_tail(log: &CommitLog<T>) -> Cursor<T> {
        log.new_tail_cursor()
    }
}

impl<T> Cursor<T> {
    pub fn new_index(&self) -> Index<T> {
        Index {
            index: Weak::clone(&self.cursor),
        }
    }
}

impl<T: Clone> Cursor<T> {
    pub fn get_copy(&mut self) -> Option<T> {
        match self.cursor.upgrade() {
            Some(cursor) => cursor.read().unwrap().next.as_ref().map(|next| {
                self.cursor = Arc::downgrade(&next);
                next.read().unwrap().value.clone()
            }),
            None => {
                self.cursor = Arc::downgrade(&self.to_head);
                self.get_copy()
            }
        }
    }
}

impl<T: Clone + Debug> Debug for Cursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?} -> {:?}]",
            self.cursor
                .upgrade()
                .as_ref()
                .map(|c| c.read().unwrap().value.clone()),
            self.cursor.upgrade().as_ref().map(|c| {
                c.read()
                    .unwrap()
                    .next
                    .as_ref()
                    .map(|n| n.read().unwrap().value.clone())
            })
        )
    }
}

pub struct CommitLog<T> {
    to_head: Link<T>,
    tail: Node<T>,
    length: usize,
}

impl<T: Default> CommitLog<T> {
    pub fn new() -> Self {
        CommitLog {
            to_head: Element::new(Default::default()),
            tail: None,
            length: 0,
        }
    }
}

impl<T> CommitLog<T> {
    pub fn new_head_cursor(&self) -> Cursor<T> {
        Cursor {
            cursor: Arc::downgrade(&self.to_head),
            to_head: Arc::clone(&self.to_head),
        }
    }

    pub fn new_tail_cursor(&self) -> Cursor<T> {
        match self.tail.as_ref() {
            Some(tail) => Cursor {
                cursor: Arc::downgrade(&tail),
                to_head: Arc::clone(&self.to_head),
            },
            None => self.new_head_cursor(),
        }
    }

    pub fn empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn append(&mut self, value: T) {
        let element = Element::new(value);
        let new_tail = Some(Arc::clone(&element));
        match self.tail.take() {
            Some(old_tail) => {
                let mut old_tail = old_tail.write().unwrap();
                old_tail.next = new_tail;
            }
            None => {
                let mut to_head = self.to_head.write().unwrap();
                to_head.next = new_tail;
            }
        };
        self.tail = Some(element);
        self.length += 1;
    }

    pub fn cleanup(&mut self, expired: &Fn(&T) -> bool) {
        loop {
            let did_expire;
            match self.to_head.read().unwrap().next.as_ref() {
                Some(next) => {
                    did_expire = expired(&next.read().unwrap().value);
                    if !did_expire {
                        return;
                    }
                }
                None => return,
            }
            if did_expire {
                self.remove_head();
            }
        }
    }

    fn remove_head(&mut self) {
        let mut to_head = self.to_head.write().unwrap();
        match to_head.next.take() {
            Some(old_head) => match old_head.write().unwrap().next.take() {
                Some(new_head) => {
                    to_head.next = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            },
            None => return,
        }
        self.length -= 1;
    }
}

impl<T: Clone + Debug> Debug for CommitLog<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cursor = self.new_head_cursor();
        write!(f, "[ ")?;
        while let Some(value) = cursor.get_copy() {
            write!(f, "{:?} -> ", value)?;
        }
        write!(f, " : {:?} ]", self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut log = CommitLog::new();
        assert!(log.empty());

        log.append(1);
        log.append(2);
        log.append(3);
        log.append(4);
        assert!(!log.empty());
        assert_eq!(log.len(), 4);

        let mut cursor0 = log.new_head_cursor();
        assert_eq!(Some(1), cursor0.get_copy());
        assert_eq!(Some(2), cursor0.get_copy());
        assert_eq!(Some(3), cursor0.get_copy());
        let index0 = cursor0.new_index();
        assert_eq!(Some(3), index0.get_copy());
        assert_eq!(Some(3), index0.get_copy());
        assert_eq!(Some(4), cursor0.get_copy());
        assert_eq!(None, cursor0.get_copy());
        assert_eq!(None, cursor0.get_copy());

        let mut cursor1 = log.new_head_cursor();
        assert_eq!(Some(1), cursor1.get_copy());

        log.append(5);
        assert_eq!(Some(5), cursor0.get_copy());

        log.cleanup(&|t: &u32| t <= &5);
        assert_eq!(None, index0.get_copy());
        assert_eq!(None, cursor1.get_copy());
        log.append(6);
        assert_eq!(Some(6), cursor0.get_copy());
        assert_eq!(Some(6), cursor1.get_copy());
        assert_eq!(log.len(), 1);

        log.append(7);
        let mut cursor2 = log.new_tail_cursor();
        assert_eq!(None, cursor2.get_copy());
        log.append(8);
        assert_eq!(Some(8), cursor2.get_copy());
        assert_eq!(None, cursor2.get_copy());

        log.cleanup(&|t: &u32| t < &8);
        assert_eq!(Some(8), cursor0.get_copy());
        assert_eq!(Some(8), cursor1.get_copy());
    }
}
