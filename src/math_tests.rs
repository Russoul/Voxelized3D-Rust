
#[cfg(test)]
mod math_tests{

    use math::{factorial};
    use na::Vector3;
    

    #[test]
    fn test_simple_math(){
        assert_eq!(factorial(1 as u32), 1);
        assert_eq!(factorial(0 as u32), 1);
        assert_eq!(factorial(3 as u32), 6);

        
    }
}

