#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn s_immutable_borrow() {
        let list = vec![1, 2, 3];
        println!("Before defining closures: {:?}", list);

        let only_borrow = || println!("From closure {:?}", list);
        println!("Before closure {:?}", list);
        only_borrow();
        println!("after {:?}", list);
    }

    #[test]
    fn closure_mutable_borrow() {
        let mut list = vec![1, 2, 3];

        println!("Before defining closures: {:?}", list);

        let mut borrow_mutability = || list.push(4);
        // println!("list {:?}", list); // this create a immutable ref but borrow_mutability have a
        // mutable reference to list for that this break borrow checker rules
        borrow_mutability();
        println!("After calling closure {:?}", list);
    }

    #[test]
    fn closure_move_ownership() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let closure = move || {
            println!("From thread {:?}", list);
            list
        };

        let list = thread::spawn(closure).join().unwrap();

        println!("{:?}", list);
    }

    #[test]
    fn iterator() {
        let nums = vec![1, 2, 3];

        let mut iter = nums.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        let sum: i32 = nums.iter().sum();

        assert_eq!(6, sum);
        println!("{:?}", nums);
    }

    #[test]
    fn operation() {
        let nums = [1, 2, 3];

        let v1 = nums.iter();
        let v2: Vec<_> = v1.map(|n| n + 1).collect();

        assert_eq!(v2, [2, 3, 4]);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn capture() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
