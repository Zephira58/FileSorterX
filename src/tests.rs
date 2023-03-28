#[cfg(test)]
mod tests {

    use std::{
        fs::File,
        path::{Path, PathBuf},
    };
    use FileSorterX::*;

    #[test]
    fn test_create_files() {
        let dir_path = "test_dir";
        let file_names = vec!["test_file1.txt", "test_file2.txt"];

        // Create the directory if it does not exist
        std::fs::create_dir_all(dir_path).unwrap();

        for file_name in file_names {
            let file_path = format!("{}/{}", dir_path, file_name);
            File::create(&file_path).unwrap();
            assert!(Path::new(&file_path).exists());
        }

        // Clean up
        std::fs::remove_dir_all(dir_path).unwrap();
    }

    #[test]
    fn test_match_extension() {
        let file_path = PathBuf::from("example.jpg");
        assert_eq!(match_extension(file_path), "sorted/pictures");

        let file_path = PathBuf::from("example.mp4");
        assert_eq!(match_extension(file_path), "sorted/videos");

        let file_path = PathBuf::from("example.wma");
        assert_eq!(match_extension(file_path), "sorted/audio");

        let file_path = PathBuf::from("example.gif");
        assert_eq!(match_extension(file_path), "sorted/gifs");

        let file_path = PathBuf::from("example.zip");
        assert_eq!(match_extension(file_path), "sorted/files");

        let file_path = PathBuf::from("example.txt");
        assert_eq!(match_extension(file_path), "sorted/documents");
    }
}
