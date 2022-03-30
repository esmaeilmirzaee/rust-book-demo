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

#[test]
fn filters_by_size() {
    use iterators::{Shoe, shoes_in_size};
    let my_shoes = vec![
        Shoe{
            size: 10,
            style: String::from("Sneaker"),
        },
        Shoe{
            size: 13,
            style: "Sandal".to_string(),
        },
        Shoe{
            size: 10,
            style: "Boot".to_string(),
        },
    ];

    let in_my_size = shoes_in_size(my_shoes, 10);
    assert_eq!(in_my_size, vec![Shoe{size: 10, style:"Sneaker".to_string()}, Shoe{size:10, style:"Boot"
        .to_string()}]);
}
