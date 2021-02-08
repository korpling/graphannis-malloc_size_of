use crate::{MallocSizeOf, MallocSizeOfOps};

mod memory_estimation;

#[cfg(feature = "smartstring")]
#[test]
fn size_smartstring() {
    use smartstring::alias::String; 
    
    let mut mem_ops =
            MallocSizeOfOps::new(memory_estimation::platform::usable_size, None, None);

    let small_string : String = "abc".into();
    assert_eq!(true, small_string.is_inline());
    assert_eq!(0, small_string.size_of(&mut mem_ops));

    let large_string : String = "ABC".repeat(1_000).into();
    assert_eq!(false, large_string.is_inline());
    assert_eq!(3_000, large_string.size_of(&mut mem_ops));
}