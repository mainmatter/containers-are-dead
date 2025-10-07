extern crate components;

unsafe extern "C" {
    #[link_name = "add-random"]
    fn add_random(num: u64) -> u64;
}

#[test]
fn verify_it_works() {
    assert!(unsafe { add_random(10) } >= 10);
}
