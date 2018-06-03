#[cfg(test)]
mod tests;

use std::fmt;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock, Weak};

type Shared<T> = Arc<RwLock<T>>;

type WeakShared<T> = Weak<RwLock<T>>;

struct Element<T> {
    value: T,
    next: Node<T>,
}

impl<T> Element<T> {
    fn new(value: T) -> Shared<Self> {
        Arc::new(RwLock::new(Self { value, next: None }))
    }
}

type Link<T> = Shared<Element<T>>;

type Pointer<T> = WeakShared<Element<T>>;

type Node<T> = Option<Link<T>>;

pub struct Index<T> {
    index: Pointer<T>,
}

impl<T> Index<T> {
    pub fn new(cursor: &Cursor<T>) -> Self {
        Self {
            index: Weak::clone(&cursor.cursor),
        }
    }
}

impl<T: Clone> Index<T> {
    pub fn get(&self) -> Option<T> {
        self.index
            .upgrade()
            .map(|i| i.read().unwrap().value.clone())
    }
}

impl<T: Clone + Debug> Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index {{ {:?} }}", self.get())
    }
}

pub struct Cursor<T> {
    cursor: Pointer<T>,
    next_index: usize,
    to_head: Pointer<T>,
    to_head_index: Arc<AtomicUsize>,
}

impl<T> Cursor<T> {
    pub fn new_head(log: &CommitLog<T>) -> Self {
        Cursor {
            cursor: Arc::downgrade(&log.to_head),
            next_index: log.to_head_index.load(Ordering::SeqCst),
            to_head: Arc::downgrade(&log.to_head),
            to_head_index: Arc::clone(&log.to_head_index),
        }
    }

    pub fn new_tail(log: &CommitLog<T>) -> Self {
        match log.tail.as_ref() {
            Some(tail) => Cursor {
                cursor: Arc::downgrade(tail),
                next_index: log.to_head_index.load(Ordering::SeqCst) + log.len(),
                to_head: Arc::downgrade(&log.to_head),
                to_head_index: Arc::clone(&log.to_head_index),
            },
            None => Cursor::new_head(log),
        }
    }

    pub fn next_index(&self) -> usize {
        self.next_index
    }
}

impl<T: Clone> Cursor<T> {
    pub fn next(&mut self) -> Option<T> {
        match self.cursor.upgrade() {
            Some(cursor) => cursor.read().unwrap().next.as_ref().map(|next| {
                self.next_index += 1;
                self.cursor = Arc::downgrade(next);
                next.read().unwrap().value.clone()
            }),
            None => {
                self.next_index = self.to_head_index.load(Ordering::SeqCst);
                self.cursor = Weak::clone(&self.to_head);
                self.next()
            }
        }
    }

    pub fn peek(&self) -> Option<T> {
        match self.cursor.upgrade() {
            Some(cursor) => cursor
                .read()
                .unwrap()
                .next
                .as_ref()
                .map(|next| next.read().unwrap().value.clone()),
            None => None,
        }
    }
}

impl<T: Clone + Debug> Debug for Cursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cursor {{ {:?} }}", self.peek())
    }
}

pub struct CommitLog<T> {
    to_head: Link<T>,
    tail: Node<T>,
    length: usize,
    to_head_index: Arc<AtomicUsize>,
}

impl<T: Default> CommitLog<T> {
    pub fn new() -> Self {
        Self {
            to_head: Element::new(Default::default()),
            tail: None,
            length: 0,
            to_head_index: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<T> CommitLog<T> {
    #[allow(dead_code)]
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

    pub fn cleanup(&mut self, expired: &Fn(&T) -> bool) -> usize {
        let mut count = 0;
        loop {
            let did_expire;
            match self.to_head.read().unwrap().next.as_ref() {
                Some(next) => {
                    did_expire = expired(&next.read().unwrap().value);
                    if !did_expire {
                        return count;
                    }
                }
                None => {
                    return count;
                }
            }
            if did_expire {
                count += 1;
                self.to_head_index.fetch_add(1, Ordering::SeqCst);
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

impl<T> Drop for CommitLog<T> {
    fn drop(&mut self) {
        let mut node = self.to_head.write().unwrap().next.take();
        while let Some(link) = node {
            node = link.write().unwrap().next.take();
        }
    }
}

impl<T: Clone + Debug> Debug for CommitLog<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cursor = Cursor::new_head(self);
        write!(f, "CommitLog {{ ")?;
        while let Some(value) = cursor.next() {
            write!(f, "{:?} -> ", value)?;
        }
        write!(f, " : {:?} }}", self.length)
    }
}
