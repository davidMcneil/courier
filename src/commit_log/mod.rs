//! A commit log like data structure.
//!
//! The [CommitLog](commit_log::CommitLog) is essentially a singly linked list. It allows appending
//! things to its tail and then running a cleanup function that removes things from its head. It
//! allows creating [Cursor](commit_log::Cursor)s which can be used to walk along the
//! elements of the commit log as well as an [Index](commit_log::Index) which points to a single
//! element of the [CommitLog](commit_log::CommitLog).

#[cfg(test)]
mod tests;

use std::fmt;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

use parking_lot::RwLock;

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

/// An index into an element of a [CommitLog]().
pub struct Index<T> {
    index: Pointer<T>,
}

impl<T> Index<T> {
    /// Create a new index at the element the [Cursor]() is currently pointing.
    pub fn new(cursor: &Cursor<T>) -> Self {
        Self {
            index: Weak::clone(&cursor.cursor),
        }
    }
}

impl<T: Clone> Index<T> {
    /// Try and get the value of the element at the index.
    ///
    /// If it returns None it means the element has been cleaned up.
    pub fn get(&self) -> Option<T> {
        self.index.upgrade().map(|i| i.read().value.clone())
    }
}

impl<T: Clone + Debug> Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index {{ {:?} }}", self.get())
    }
}

/// A cursor which can be used to walk the elements of a [CommitLog]().
pub struct Cursor<T> {
    cursor: Pointer<T>,
    next_index: usize,
    to_head: Pointer<T>,
    to_head_index: Arc<AtomicUsize>,
}

impl<T> Cursor<T> {
    /// Create a new cursor at the head (beginning) of the [CommitLog]().
    pub fn new_head(log: &CommitLog<T>) -> Self {
        Cursor {
            cursor: Arc::downgrade(&log.to_head),
            next_index: log.to_head_index.load(Ordering::SeqCst),
            to_head: Arc::downgrade(&log.to_head),
            to_head_index: Arc::clone(&log.to_head_index),
        }
    }

    /// Create a new cursor at the tail (end) of the [CommitLog]().
    pub fn new_tail(log: &CommitLog<T>) -> Self {
        match log.tail.as_ref() {
            Some(tail) => Cursor {
                cursor: Arc::downgrade(tail),
                next_index: log.to_head_index.load(Ordering::SeqCst) + log.len(),
                to_head: Arc::downgrade(&log.to_head),
                to_head_index: Arc::clone(&log.to_head_index),
            },
            // The commit log does not have a tail element so simply use its head.
            None => Cursor::new_head(log),
        }
    }

    /// Get the index of the next element the cursor will get.
    pub fn next_index(&self) -> usize {
        self.next_index
    }
}

impl<T: Clone> Cursor<T> {
    /// Get the value of the next element of the cursor.
    ///
    /// If it returns None it means the cursor has reached the tail of the [CommitLog]().
    pub fn next(&mut self) -> Option<T> {
        match self.cursor.upgrade() {
            // If the cursor is pointing to a valid element, see if it has a next element and if it
            // does return its value and increment the cursor. If it does not have a next element,
            // it means we have reached the tail of the commit log.
            Some(cursor) => cursor.read().next.as_ref().map(|next| {
                self.next_index += 1;
                self.cursor = Arc::downgrade(next);
                next.read().value.clone()
            }),
            // If the cursor has expired it means we are pointing to a cleaned up element.
            // Promote the commit log's head to be the cursor and try calling next again.
            None => {
                if let Some(_) = self.to_head.upgrade() {
                    self.next_index = self.to_head_index.load(Ordering::SeqCst);
                    self.cursor = Weak::clone(&self.to_head);
                    self.next()
                } else {
                    // This should be an unreachable state as the commit log should always have a
                    // valid to head, but for now simply return None.
                    None
                }
            }
        }
    }

    /// Peek at the value of the next element of the cursor without progressing the cursor.
    ///
    /// If it returns None it means the cursor has reached the tail of the [CommitLog]().
    pub fn peek(&self) -> Option<T> {
        match self.cursor.upgrade() {
            Some(cursor) => cursor
                .read()
                .next
                .as_ref()
                .map(|next| next.read().value.clone()),
            None => None,
        }
    }
}

impl<T: Clone + Debug> Debug for Cursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cursor {{ {:?} }}", self.peek())
    }
}

/// A commit log like data structure.
///
/// Allows pushing elements to its head and cleaning up elements from its tail.
pub struct CommitLog<T> {
    to_head: Link<T>,
    tail: Node<T>,
    length: usize,
    to_head_index: Arc<AtomicUsize>,
}

impl<T: Default> CommitLog<T> {
    /// Create a new commit log
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
    /// Check if there are not elements
    #[allow(dead_code)]
    pub fn empty(&self) -> bool {
        self.length == 0
    }

    /// Get the length
    pub fn len(&self) -> usize {
        self.length
    }

    /// Add an element to the tail
    pub fn append(&mut self, value: T) {
        let element = Element::new(value);
        let new_tail = Some(Arc::clone(&element));
        match self.tail.take() {
            // Point the current tails next to the new tail
            Some(old_tail) => {
                let mut old_tail = old_tail.write();
                old_tail.next = new_tail;
            }
            // There is not tail so point head to the new tail
            None => {
                let mut to_head = self.to_head.write();
                to_head.next = new_tail;
            }
        };
        self.tail = Some(element);
        self.length += 1;
    }

    /// Remove elements from the head
    ///
    /// Given a function expired, elements will be removed from the tail as long as expired returns
    /// true once expired returns false cleanup will exit.
    pub fn cleanup(&mut self, expired: &Fn(&T) -> bool) -> usize {
        let mut count = 0;
        loop {
            // Check if the head has expired
            let did_expire;
            match self.to_head.read().next.as_ref() {
                Some(next) => {
                    did_expire = expired(&next.read().value);
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
                self.remove_head();
            }
        }
    }

    fn remove_head(&mut self) {
        let mut to_head = self.to_head.write();
        // Take ownership of head element. This means it will be dropped at the end of its lifetime.
        match to_head.next.take() {
            // Take ownership of the head element's next element
            Some(old_head) => match old_head.write().next.take() {
                Some(new_head) => {
                    to_head.next = Some(new_head);
                }
                // The last element of the commit log was removed
                None => {
                    self.tail = None;
                }
            },
            // The commit log is empty
            None => return,
        }
        self.to_head_index.fetch_add(1, Ordering::SeqCst);
        self.length -= 1;
    }
}

impl<T> Drop for CommitLog<T> {
    // The generated drop method is a recursive algorithm which makes it easy to blow the stack
    // this overrides with a iterative algorithm.
    fn drop(&mut self) {
        let mut node = self.to_head.write().next.take();
        while let Some(link) = node {
            node = link.write().next.take();
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
