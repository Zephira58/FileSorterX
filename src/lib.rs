use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fs, io::Write};

use self_update::cargo_crate_version;

pub fn create_files() {
    let extention_array = [
        "jpg", "mp4", "wma", "gif", "zip", "txt", "torrent", "iso", "ttf", "dll", "exe", "",
    ];
    for x in 0..10001 {
        fs::File::create(format!("{}.{}", x, extention_array[x % 11])).unwrap();
    }
}

pub fn match_extension(path: PathBuf) -> &'static str {
    let directory = match path.extension() {
        // Pictures
        Some(ext) if ext == "jpg" => "sorted/pictures",
        Some(ext) if ext == "jpeg" => "sorted/pictures",
        Some(ext) if ext == "png" => "sorted/pictures",
        Some(ext) if ext == "psd" => "sorted/pictures",
        Some(ext) if ext == "svg" => "sorted/pictures",
        Some(ext) if ext == "ai" => "sorted/pictures",
        Some(ext) if ext == "bmp" => "sorted/pictures",
        // Videos
        Some(ext) if ext == "mp4" => "sorted/videos",
        Some(ext) if ext == "mkv" => "sorted/videos",
        Some(ext) if ext == "avi" => "sorted/videos",
        Some(ext) if ext == "webm" => "sorted/videos",
        Some(ext) if ext == "mov" => "sorted/videos",
        // Music
        Some(ext) if ext == "mp3" => "sorted/audio",
        Some(ext) if ext == "ogg" => "sorted/audio",
        Some(ext) if ext == "wma" => "sorted/audio",
        Some(ext) if ext == "wav" => "sorted/audio",
        // Gifs
        Some(ext) if ext == "gif" => "sorted/gifs",
        Some(ext) if ext == "apng" => "sorted/gifs",
        // Files
        Some(ext) if ext == "zip" => "sorted/files",
        Some(ext) if ext == "rar" => "sorted/files",
        Some(ext) if ext == "tar" => "sorted/files",
        Some(ext) if ext == "7z" => "sorted/files",
        Some(ext) if ext == "gz" => "sorted/files",
        Some(ext) if ext == "cfg" => "sorted/files",
        // Doccuments
        Some(ext) if ext == "txt" => "sorted/documents",
        Some(ext) if ext == "pdf" => "sorted/documents",
        Some(ext) if ext == "doc" => "sorted/documents",
        Some(ext) if ext == "docx" => "sorted/documents",
        Some(ext) if ext == "htm" => "sorted/documents",
        Some(ext) if ext == "ppt" => "sorted/documents",
        Some(ext) if ext == "pptx" => "sorted/documents",
        Some(ext) if ext == "clsx" => "sorted/documents",
        Some(ext) if ext == "xlsx" => "sorted/documents",
        Some(ext) if ext == "accdb" => "sorted/documents",
        Some(ext) if ext == "csv" => "sorted/documents",
        Some(ext) if ext == "epub" => "sorted/documents",
        // Torrents
        Some(ext) if ext == "torrent" => "sorted/torrents",
        // System Images
        Some(ext) if ext == "iso" => "sorted/system-images",
        // Fonts
        Some(ext) if ext == "fnt" => "sorted/fonts",
        Some(ext) if ext == "fon" => "sorted/fonts",
        Some(ext) if ext == "otf" => "sorted/fonts",
        Some(ext) if ext == "ttf" => "sorted/fonts",
        // Programming
        Some(ext) if ext == "py" => "sorted/programming/python",
        Some(ext) if ext == "rs" => "sorted/programming/rust",
        Some(ext) if ext == "js" => "sorted/programming/javaScript",
        Some(ext) if ext == "jar" => "sorted/programming/java",
        Some(ext) if ext == "html" => "sorted/programming/hml",
        Some(ext) if ext == "c" => "sorted/programming/c",
        Some(ext) if ext == "cpp" => "sorted/programming/c++",
        Some(ext) if ext == "cs" => "sorted/programming/c#",
        Some(ext) if ext == "go" => "sorted/programming/go",
        Some(ext) if ext == "swift" => "sorted/programming/swift",
        Some(ext) if ext == "php" => "sorted/programming/php",
        Some(ext) if ext == "r" => "sorted/programming/r",
        Some(ext) if ext == "json" => "sorted/programming/json",
        Some(ext) if ext == "dll" => "sorted/programming/dll",
        Some(ext) if ext == "md" => "sorted/programming/markdown",
        // Applications
        Some(ext) if ext == "msi" => "sorted/applications",
        Some(ext) if ext == "apk" => "sorted/applications",
        Some(ext) if ext == "exe" => "sorted/applications",
        Some(ext) if ext == "appimage" => "sorted/applications",
        //Games
        Some(ext) if ext == "osz" => "sorted/game-stuff/osu",
        Some(ext) if ext == "osk" => "sorted/game-stuff/osu",
        //Encyption
        Some(ext) if ext == "gpg" => "sorted/encryption",
        Some(ext) if ext == "pcv" => "sorted/encryption",
        Some(ext) if ext == "enc" => "sorted/encryption",
        _ => "sorted/other",
    };
    directory
}

pub fn write_logfile(file_name: &OsStr, moveto_directory: &Path) -> bool {
    let mut logfile = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("sorter-logs/logs.txt")
        .expect("create failed");

    logfile
        .write_all(format!("{:?}", file_name).as_bytes())
        .expect("write failed");
    logfile
        .write_all(" Moved to ".as_bytes())
        .expect("write failed");
    logfile
        .write_all(format!("{:?}\n", moveto_directory.display()).as_bytes())
        .expect("write failed");

    true
}

pub fn sort_files(verbose: bool, log: bool) -> std::io::Result<()> {
    loop {
        let entries = fs::read_dir("./").unwrap();

        for index in entries {
            let path = index.unwrap().path();
            let file_name = match path.file_name() {
                Some(file_name) => file_name,
                None => continue,
            };

            let moveto_directory = Path::new(match_extension(path.clone()));
            fs::create_dir_all(moveto_directory).unwrap();
            fs::rename(&path, moveto_directory.join(file_name))?;

            if verbose {
                println!("{:?} moved to {:?}", file_name, moveto_directory.display());
            }

            if log {
                let log_dir = "sorter-logs";
                fs::create_dir_all(log_dir).unwrap();
                write_logfile(file_name, moveto_directory);
            }
        }
    }
}

pub fn update_filesorterx() -> Result<(), Box<dyn (::std::error::Error)>> {
    println!("Updating FileSorterX to the latest version...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("xanthus58")
        .repo_name("FileSorterX")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
