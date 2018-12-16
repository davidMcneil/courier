use crate::commit_log::*;
use rand::{thread_rng, Rng};

#[test]
fn commit_log_basic() {
    let mut log = CommitLog::new();
    assert!(log.empty());
    log.cleanup(&|_: &u32| true);

    let mut cursor0 = Cursor::new_tail(&log);

    log.append(1);
    log.append(2);
    log.append(3);
    log.append(4);
    println!("{:?}", log);
    assert!(!log.empty());
    assert_eq!(log.len(), 4);

    println!("{:?}", cursor0);
    assert_eq!(Some(1), cursor0.peek());
    assert_eq!(Some(1), cursor0.next());
    assert_eq!(Some(2), cursor0.peek());
    assert_eq!(Some(2), cursor0.next());
    assert_eq!(Some(3), cursor0.peek());
    assert_eq!(Some(3), cursor0.next());
    let index0 = Index::new(&cursor0);
    println!("{:?}", index0);
    assert_eq!(Some(3), index0.get());
    assert_eq!(Some(3), index0.get());
    assert_eq!(Some(4), cursor0.next());
    assert_eq!(None, cursor0.next());
    assert_eq!(None, cursor0.peek());
    assert_eq!(None, cursor0.next());

    let mut cursor1 = Cursor::new_head(&log);
    assert_eq!(Some(1), cursor1.next());

    log.append(5);
    assert_eq!(Some(5), cursor0.next());

    log.cleanup(&|t: &u32| t <= &5);
    assert_eq!(None, index0.get());
    assert_eq!(None, cursor1.peek());
    assert_eq!(None, cursor1.next());
    log.append(6);
    assert_eq!(Some(6), cursor0.next());
    assert_eq!(Some(6), cursor1.next());
    assert_eq!(log.len(), 1);

    log.append(7);
    let mut cursor2 = Cursor::new_tail(&log);
    assert_eq!(None, cursor2.next());
    log.append(8);
    assert_eq!(Some(8), cursor2.next());
    assert_eq!(None, cursor2.next());

    log.cleanup(&|t: &u32| t < &8);
    assert_eq!(Some(8), cursor0.next());
    assert_eq!(Some(8), cursor1.next());
}

#[test]
fn commit_log_random() {
    for _ in 0..100 {
        let mut rng = thread_rng();
        let mut log = CommitLog::new();
        let mut cursor = Cursor::new_head(&log);
        let mut r1;
        let mut r2 = 0;
        let mut expected = Vec::new();
        for _ in 0..100 {
            r1 = rng.gen_range(r2, r2 + 100);
            r2 = rng.gen_range(r1 + 50, r1 + 150);
            for i in r1..r2 {
                log.append(i);
                expected.push(i);
            }
            let num_nexts = rng.gen_range(0, r2 - r1);
            for i in 0..num_nexts {
                assert_eq!(Some(expected[i]), cursor.next());
            }
            expected = expected.split_off(num_nexts);
            let cleanup_threshold = rng.gen_range(r1, r2);
            log.cleanup(&|t: &usize| t <= &cleanup_threshold);
            expected.retain(|&t| t > cleanup_threshold);
        }
    }
}
