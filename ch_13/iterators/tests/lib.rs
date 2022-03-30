#[test]
fn iterator_demonstrators() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_consumption() {
    let v1 = vec![1,2,3,4,5];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(15, total);
}

#[test]
fn iterator_chaining() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);
    let mut i = 0;
    for item in v1_iter {
        assert_eq!(v1[i]+1, item);
        i+=1;
    }
}
