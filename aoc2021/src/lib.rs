pub mod utils {
    use std::fs;
    use std::time::Instant;

    pub fn read_file_into_lines(module_path: &str, filename: &str) -> std::io::Result<Vec<String>> {
        let filepath = "./src/bin/".to_owned() + module_path + "/" + filename;
        println!("Reading file {}", filepath);
        let content = fs::read_to_string(filepath)?;
        Ok(content.lines().map(|s| s.to_string()).collect())
    }

    pub fn lines_to_usize_vec(lines: &Vec<String>) -> Vec<usize> {
        lines.iter().map(|s| s.parse::<usize>().unwrap()).collect()
    }

    pub fn measure_and_print<T>(part: u8, f: fn(&T) -> Option<i64>, arg: &T) {
        let start = Instant::now();
        let result = f(arg);
        let duration = start.elapsed();
        match result {
            Some(answer) => println!("Part {} took {:?} to find: {}", part, duration, answer),
            None => println!("Part {} took {:?} to find no solution", part, duration),
        }
    }

    pub fn delim_line_into_usize(s: &str, delim: char) -> Vec<usize> {
        let line_split: Vec<&str> = s.split(delim).collect();
        line_split
            .iter()
            .filter_map(|&c| c.parse::<usize>().ok())
            .collect()
    }
}
