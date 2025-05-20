use tokio::fs;
use std::env;
pub async fn get_data_file() -> Result<String, std::io::Error> {
    let file_path: &str = "./input_test_case.txt";
        let current_dir = env::current_dir()?.to_string_lossy().to_string();
        if !std::path::Path::new(file_path).exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", file_path),
            ));
        }
    
    let contents: String = fs::read_to_string(file_path).await?;
    Ok(contents)
}







