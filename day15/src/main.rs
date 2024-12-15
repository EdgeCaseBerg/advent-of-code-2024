pub mod boilerplate;


fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    println!("{:?}", raw_data);
}

mod main_tests {
    #[test]
    fn we_can_test_the_program() {
        assert_eq!(1, 1);
    }
}