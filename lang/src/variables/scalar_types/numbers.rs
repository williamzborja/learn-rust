#[cfg(test)]
mod test {
    use std::mem::{size_of, size_of_val};

    #[test]
    fn test_scalar() {
        let num: i32 = 12;

        println!("{:b}", num);

        println!("u128 MIN:{}", std::u128::MIN);
        println!("u128 MAX:{}", std::u128::MAX);

        let c = 'c';

        println!("{}", c);

        let c = '😻';
        let size = size_of::<char>();
        println!("size of characther is : {}", size);
        println!("size of this emoji is {}", size_of_val(&c));
    }

    #[test]
    fn test_more() {
        let long_num = 100_000_000;
        let (num, digits) = (1, 2.3);

        println!("{num} {digits} {long_num}");

        println!("{} {} {}", 28u128, 3u8, -3i8);
    }
}
