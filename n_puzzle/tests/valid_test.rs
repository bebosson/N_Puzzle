pub mod valid_tests
{
    use n_puzzle::solver::mod_test::mod_test::test_solver_valid_3;

    #[test]
    pub fn test_valid_bad_aligned()
    {
        test_solver_valid_3("tests/good_grid/bad_aligned".to_string(), 3, 0);
    }
    #[test]
    pub fn test_valid_comment_input()
    {
        test_solver_valid_3("tests/good_grid/comment_input".to_string(), 3, 0);
    }

    #[test]
    pub fn test_valid_exemple_grid()
    {
        test_solver_valid_3("tests/good_grid/exemple_grid".to_string(), 3, 0);
    }

    #[test]
    pub fn test_valid_strange_comment_input()
    {
        test_solver_valid_3("tests/good_grid/strange_comment_input".to_string(), 3, 0);
    }
}