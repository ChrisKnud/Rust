use std::{
    fs::{self, File},
    io::{BufReader, Read, Write}, env, ffi::OsStr,
};

// Create new cargo directory
fn make_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

fn make_file(dir: &str, name: &str, content: &[u8]) -> std::io::Result<File> {
    let mut file = File::create(&format!("{}/{}", dir, name))?;
    file.write_all(content)?;
    Ok(file)
}

fn file_to_string(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {

    // Set enviroment variable
    let env_key = "EMBED_RUST";
    let env_path: String = env::var(env_key).unwrap();
    println!("env: {}", env_path);


    // Directories
    let paths: [&str; 3] = [".cargo", ".vscode", "."]; // Directory paths

    // Files
    let cargo_files = vec!["config.toml"];
    let vs_files = vec!["extensions.json", "launch.json", "tasks.json"];
    let workdir_files = vec!["memory.x", "openocd.cfg", "openocd.gdb", ".gitignore"];

    // Vector with files sorted after directory
    let files = vec![cargo_files, vs_files, workdir_files];

    // Files path to files containing the content
    // That will be written to the new files
    let file_paths: [&str; 3] = [
        &format!("{}{}", env_path.as_str(), ".\\config_files\\cargo_files\\"),
        &format!("{}{}", env_path.as_str(), ".\\config_files\\vs_files\\"),
        &format!("{}{}", env_path.as_str(), ".\\config_files\\workdir_files\\"),
    ];

    let mut content = String::new(); // Temporary variable used to share content between files.

    // Make directories and files
    // as well as writing the files contents.
    for i in 0..paths.len() {
        
        // Check if paths[i] contains a value
        if i < paths.len() {

            // Makes directory
            match make_dir(paths[i]) {
            Ok(_) => println!("Successfully created directory."),
            Err(e) => eprintln!("Error when creating directory: {:?}.", e),
        }

        }
       
        // Makes each file that are going to be
        // contained by the directory
        for file in 0..files[i].len() {
            // Get the content that will be
            // written to the file
            match file_to_string(&format!(
                "{}{}",
                file_paths[i], files[i][file]
            )) {
                Ok(cont) => content = cont,
                Err(e) => println!("Error: {:?}", e),
            }

            // Make the new file and write it's contents
            match make_file(paths[i], files[i][file], content.as_bytes()) {
                Ok(_) => println!("Successfully created file {}.", files[i][file]),
                Err(e) => eprintln!("Error when creating file: {}. {:?}.", files[i][file], e),
            }
        }
    }
}
