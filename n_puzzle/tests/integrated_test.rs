#[cfg(test)]
pub mod test{

    use n_puzzle::solver::mod_test::mod_test::{*};


    #[test]
    fn test_valid_3_manhattan()
    {
        test_solver_valid_3("target/stest_".to_string(), 3, 0);
    }

    #[test]
    fn test_valid_3_euclidien()
    {
        test_solver_valid_3("target/stest_".to_string(), 3, 1);
    }

    #[test]
    fn test_valid_3_tiles_out_of_place()
    {
        test_solver_valid_3("target/stest_".to_string(), 3, 2);
    }

    #[test]
    fn test_invalid_3_manhattan()
    {
        test_solver_invalid_3("target/utest_".to_string(), 3, 0);
    }

    #[test]
    fn test_invalid_3_euclidien()
    {
        test_solver_invalid_3("target/utest_".to_string(), 3, 1);
    }

    #[test]
    fn test_invalid_3_tiles_out_of_place()
    {
        test_solver_invalid_3("target/utest_".to_string(), 3, 2);
    }
    
    #[test]
    fn test_valid_4_manhattan()
    {
        test_solver_valid_4("target/stest_".to_string(), 4, 0);
    }

    #[test]
    fn test_valid_4_euclidien()
    {
        test_solver_valid_4("target/stest_".to_string(), 4, 1);
    }

    #[test]
    fn test_valid_4_tiles_out_of_place()
    {
        test_solver_valid_4("target/stest_".to_string(), 4, 2);
    }

    #[test]
    fn test_invalid_4_manhattan()
    {
        test_solver_invalid_4("target/utest_".to_string(), 4, 0);
    }

    #[test]
    fn test_invalid_4_euclidien()
    {
        test_solver_invalid_4("target/utest_".to_string(), 4, 1);
    }

    #[test]
    fn test_invalid_4_tiles_out_of_place()
    {
        test_solver_invalid_4("target/utest_".to_string(), 4, 2);
    }
   
}