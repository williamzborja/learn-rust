#[cfg(test)]
mod test {
    use std::collections::LinkedList;

    #[test]
    fn test_linked_list() {
        let mut list: LinkedList<String> = LinkedList::new();

        list.push_back("Hello".to_string());
        list.push_back("World".to_string());

        let front = list.front();
        let back = list.back();
        println!("List: {:?}", list);

        println!("front:{:?}", front.unwrap());
        println!("back: {:?}", back.unwrap());

        list.pop_front();
        list.pop_back();
        println!("{}", "-".repeat(20));
        println!("List: {:?}", list);
    }
}
