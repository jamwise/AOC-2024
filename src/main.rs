fn main() {
    let bin_folder = std::fs::read_dir("src/bin").unwrap();
    let mut advent_days = Vec::new();

    for entry in bin_folder {
        let entry = entry.unwrap();
        let path = entry.path();
        let folder_name = path.file_name().unwrap().to_str().unwrap().to_string();
        advent_days.push(folder_name);
    }

    let mut advent_days = advent_days
        .iter()
        .filter(|x| x.parse::<i32>().is_ok())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    advent_days.sort();

    println!("All advent of code 2024 results\n");

    for day in advent_days {
        let output = std::process::Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(&day.to_string())
            .output()
            .expect("failed to execute process");

        println!("Day {}:", day);
        println!("{} \n", String::from_utf8_lossy(&output.stdout));
    }
}
