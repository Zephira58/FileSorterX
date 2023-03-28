![visitors](https://visitor-badge.glitch.me/badge?page_id=kogisin/Xanthus58$Xanthus58$FuleSorterX)

# Welcome to the FileSorterX
FileSorterX is a small command line application that automatically sorts files based on their file extension and places them into the appropriate folder. The application is written in Rust, which makes it very fast `It only takes 6 seconds to sort 10,000 files!` As well as being very reliable and efficient.

## How to Use
To use FileSorterX, simply download the binary executable file for your operating system or run the command `cargo install FileSorterX` and run it from the command line in the location you want to sort. The application will scan the current directory for any files and automatically sort them based on their file extension. Or create over 10,000 random files with 0 data to test with. For example, if you have a file called document.pdf in the current directory, FileSorterX will create a folder called Documents and move the file into that folder.

Both rust developers and cli users can specify custom file sorting! Just simple input the directory to read from; the directory you want your files matching the extention to go. And the file extention itself. An example command would be `FileSorterX --xsorter --input ~\Downloads --output ~\Downloads\test --extension txt`

### System Requirements
FileSorterX is supported on all operating systems. You'll need to have Rust installed on your system in order to compile the source code and generate the binary executable. Or install it via cargo

### Contributing
If you'd like to contribute to the Rust File Sorter project, please feel free to fork the repository and submit a pull request. We welcome contributions of all kinds, including bug fixes, feature enhancements, and documentation improvements.

### License
The Rust File Sorter is released under the MIT License. See the LICENSE file for more information.
