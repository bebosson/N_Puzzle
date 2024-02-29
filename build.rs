use std::process::Command;

fn create_folder_test()
{
    let output = Command::new("sh")
    .current_dir(".")
    .arg("scripts/create_folder_grid.sh")
    .output();
    println!("{:?}", output);
}


fn main()
{
    create_folder_test()
}
