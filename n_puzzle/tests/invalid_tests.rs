#[cfg(test)]
pub mod invalid_tests
{
    use n_puzzle::solver::mod_test::mod_test::{test_solver_invalid_3, test_solver_invalid_4};

    #[test]
    #[should_panic]
    pub fn test_invalid_bad_size()
    {
        test_solver_invalid_4("tests/bad_grid/bad_size".to_string(), 4, 0);
    }
    #[test]
    #[should_panic]
    pub fn test_invalid_duplicate()
    {
        test_solver_invalid_3("tests/bad_grid/duplicate".to_string(), 3, 0);
    }
    #[test]
    #[should_panic]
    pub fn test_invalid_bad_format()
    {
        test_solver_invalid_3("tests/bad_grid/bad_format".to_string(), 3, 0);
    }
    #[test]
    #[should_panic]
    pub fn test_invalid_bad_input_str()
    {
        test_solver_invalid_3("tests/bad_grid/bad_input_str".to_string(), 3, 0);
    }
    #[test]
    #[should_panic]
    pub fn test_invalid_out_of_range()
    {
        test_solver_invalid_3("tests/bad_grid/out_of_range".to_string(), 3, 0);
    }
    #[test]
    #[should_panic]
    pub fn test_invalid_bad_size_too_little()
    {
        test_solver_invalid_3("tests/bad_grid/bad_size_too_little".to_string(), 3, 0);
    }
}