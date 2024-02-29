pub mod mod_test
{
    use crate::solver::resolve::resolve::parse_and_play;

    pub fn correct_path(rel_path: &String, size: u32, num_file: u32) -> String
    {
        let path: String;

        path = rel_path.clone()+ &size.to_string() + &"/".to_string() + &"test_".to_string() + &num_file.to_string() + &".txt".to_string();
        path
    }

    pub fn test_solver_valid_3(rel_path : String, size: u32, d: u32)
    {
        let d_str = d.to_string();
        let t: String;
        if size > 3 || (size == 3 && d == 2) {t = "n".to_string();}
        else {t = "y".to_string();}
        
        for i in 1..5
        {
            assert_eq!(true, parse_and_play(["path".to_string(), d_str.clone(), correct_path(&rel_path, size, i), t.clone()].to_vec()).is_ok());
        }
    }

    pub fn test_solver_invalid_3(rel_path : String, size: u32, d: u32)
    {
        let d_str = d.to_string();
        let t: String;
        t = "y".to_string();
        for i in 1..5
        {
            assert_eq!(true, parse_and_play(["path".to_string(), d_str.clone(), correct_path(&rel_path, size, i), t.clone()].to_vec()).is_err());
        }
    }

    pub fn test_solver_valid_4(rel_path : String, size: u32, d: u32)
    {
        let d_str = d.to_string();
        let t: String;
        if size > 3 || (size == 3 && d == 2) {t = "n".to_string();}
        else {t = "y".to_string();}
        
        assert_eq!(true, parse_and_play(["path".to_string(), d_str.clone(), correct_path(&rel_path, size, 1), t.clone()].to_vec()).is_ok());
    }

    pub fn test_solver_invalid_4(rel_path : String, size: u32, d: u32)
    {
        let d_str = d.to_string();
        let t: String;
        t = "y".to_string();
        assert_eq!(true, parse_and_play(["path".to_string(), d_str.clone(), correct_path(&rel_path, size, 1), t.clone()].to_vec()).is_err());
    }

}