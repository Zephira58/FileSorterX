/// # Alignment
/// `(extension, (type, Option(alternative_sorted_dir), Option(sorted_dir)))`
///
/// If `sorted_dir` is `None` => use extension.
///
/// ## Usage
/// ```markdown
/// mountain.tiff -> image/tif/mountain.tiff
/// mountain.tiff -> image/mountain.tiff
///
/// screamer.gif -> image/animated/screamer.gif
/// screamer.gif -> image/animated/gif/screamer.gif
/// screamer.gif -> image/gif/screamer.gif
/// screamer.gif -> image/screamer.gif
/// ```
pub static EXTENSIONS: [(&str, (&str, Option<&str>, Option<&str>)); 67] = [
    // Image
    ("bmp", ("image", None, None)),
    ("jpg", ("image", None, Some("jpg"))),
    ("jpeg", ("image", None, Some("jpg"))),
    ("png", ("image", None, None)),
    ("tif", ("image", None, Some("tif"))),
    ("tiff", ("image", None, Some("tif"))),
    ("webp", ("image", None, None)),
    ("svg", ("image", None, None)),
    // Animated image
    ("apng", ("image", Some("animated"), None)),
    ("gif", ("image", Some("animated"), None)),
    // Video
    ("mp4", ("video", None, None)),
    ("avi", ("video", None, None)),
    ("webm", ("video", None, None)),
    ("mov", ("video", None, None)),
    ("qt", ("video", None, Some("quicktime"))),
    ("mov", ("video", None, Some("quicktime"))),
    ("yuv", ("video", None, None)),
    // Program
    ("swf", ("program", None, None)),
    ("exe", ("program", None, None)),
    ("msi", ("program", None, None)),
    ("apk", ("program", Some("package"), None)),
    ("rpm", ("program", Some("package"), None)),
    ("appimage", ("program", Some("portable"), None)),
    // Audio
    ("flac", ("audio", Some("lossless"), None)),
    ("alac", ("audio", Some("lossless"), None)),
    ("wav", ("audio", Some("lossless"), None)),
    ("ape", ("audio", Some("lossless"), None)),
    ("wav", ("audio", Some("lossless"), None)),
    ("mp3", ("audio", Some("lossy"), None)),
    ("opus", ("audio", Some("lossy"), None)),
    ("aac", ("audio", Some("lossy"), None)),
    ("ogg", ("audio", Some("lossy"), None)),
    ("wma", ("audio", None, None)),
    // Project :: Adobe
    ("psd", ("adobe", Some("photoshop"), None)),
    ("ai", ("adobe", Some("illustrator"), None)),
    ("indd", ("adobe", Some("in_design"), None)),
    // Archive
    ("7z", ("archive", None, None)),
    ("tar", ("archive", None, None)),
    ("gz", ("archive", None, None)),
    ("xz", ("archive", None, None)),
    ("zip", ("archive", None, None)),
    ("rar", ("archive", None, None)),
    ("sfx", ("archive", None, None)),
    ("arc", ("archive", None, None)),
    // Documents :: Plain text
    ("txt", ("document", Some("plain_text"), None)),
    ("csv", ("document", None, None)),
    // Documents :: Markdown
    ("md", ("document", Some("markdown"), None)),
    // Documents :: HTML
    ("htm", ("document", None, Some("html"))),
    ("html", ("document", None, Some("html"))),
    // Documents :: Microsoft
    ("doc", ("document", Some("word"), None)),
    ("docx", ("document", Some("word"), None)),
    ("ppt", ("document", Some("powerpoint"), None)),
    ("pptx", ("document", Some("powerpoint"), None)),
    // Books
    ("epub", ("book", None, None)),
    ("mobi", ("book", None, None)),
    ("fb2", ("book", None, None)),
    ("cbz", ("book", None, None)),
    ("cbr", ("book", None, None)),
    ("cb7", ("book", None, None)),
    // Torrent
    ("torrent", ("torrent", None, None)),
    // Iso
    ("iso", ("disc_image", None, None)),
    // Fonts
    ("fnt", ("font", None, None)),
    ("fon", ("font", None, None)),
    ("otf", ("font", None, None)),
    ("ttf", ("font", None, None)),
    // OSU
    ("osz", ("osu", None, None)),
    ("osk", ("osu", None, None)),
];
