use std::fs;

pub fn get_data() -> String{
    let file_path:&str = "C:/Users/User MSI/Desktop/corebank/input_test_case.txt";
    let contents:String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}







