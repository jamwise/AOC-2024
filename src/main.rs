fn main() {
    let bin_folder = std::fs::read_dir("src/bin").unwrap();
    let mut advent_days = Vec::new();

    for entry in bin_folder {
        let entry = entry.unwrap();
        let path = entry.path();
        let folder_name = path.file_name().unwrap().to_str().unwrap().to_string();
        advent_days.push(folder_name);
    }

    // run each bin in the vector
    for day in advent_days {
        let output = std::process::Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(&day)
            .output()
            .expect("failed to execute process");

        println!("All advent of code 2024 results\n");
        println!("Day {}:", day);
        println!("{} \n", String::from_utf8_lossy(&output.stdout));
    }
}
