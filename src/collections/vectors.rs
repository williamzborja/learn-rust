#[cfg(test)]
mod test {
    #[test]
    fn create() {
        let mut nums = Vec::new();

        nums.push(1);
        nums.push(2);
    
        println!("numbers: {:#?} with len {}", nums, nums.len());
    }

    #[test]
    fn create_with_macro() {
        let nums = vec![1, 2];

        println!("numbers: {:#?}", nums);
    }

    #[test]
    fn access() {
        let nums = vec![1, 2];

        println!("numbers: {:#?}", nums);
        println!("first: {:?}", nums[0]);
        println!("first: {:?}", nums.get(0));
        println!("second: {:?}", nums.get(1));
    }

    #[test]
    fn iterate() {
        let nums = vec![1, 2];

        for (i, num) in nums.iter().enumerate() {
            println!("index {} num: {}", i, num);
        }
    }

    #[test]
    fn contains() {
        let nums = vec![1, 2];

        println!("numbers: {:#?}", nums);
        println!("contains 1: {}", nums.contains(&1));
        println!("contains 3: {}", nums.contains(&3));
    }

    #[test]
    fn insert(){
        let mut nums = vec![1, 2];

        println!("numbers: {:?}", nums);
        nums.insert(0, 3);
        println!("numbers: {:?}", nums);
    }

    #[test]
    fn slice_to_vec(){
        let nums = vec![1, 2, 3, 4, 5];

        let slice = &nums[1..3];

        println!("numbers: {:?}", nums);
        println!("slice: {:?}", slice);


        println!("vector: {:?}", slice.to_vec());

    }
}
