use std::{env, io::Write};

// proj new, run, build <project>

fn main() -> std::io::Result<()> {
    // ignore progarm name
    let mut cli_pattern = env::args().skip(1);
    // match by size of command line input
    match cli_pattern.len() {
        2 => {
            // len 2 new and potential future
            // idk why nth(0) twice i think it increments the iterator
            match cli_pattern.nth(0) {
                // take the stuffing out the chicken
                Some(x) => {
                    if x == "new" {
                        make_proj(cli_pattern.nth(0))?;
                    }
                }
                None => println!("issue has happened"), // if no stuffing
            }
        }
        1 => {
            match cli_pattern.nth(0) {
                Some(x) => {
                    // could add more match instead of if check
                    if x == "build" {
                        build_proj()
                    }
                    if x == "run" {
                        run_proj()
                    }
                }
                None => println!("issue has happened"),
            }
        }
        // not implemented you terrible coder
        _ => {
            println!("Not implemented yet");
        }
    }
    // to be safe for that one random std::io::Result<()>
    Ok(())
}

// going to store more stuff than just project name maybe
// like folders inside the main project one
struct Project {
    name: String,
}

// dont know if i should add as like a trait or crate or something
// for the Project class
// idk
// reminds me of a more complex C
fn make_proj(command: Option<String>) -> std::io::Result<()> {
    use std::fs;
    let project = Project {
        name: command.expect("None"),
    };
    // looks like filesystem in cpp17 such a slay
    let proj_dir = format!("{}", project.name.clone());
    // pretty much use it like pass by reference in cpp ez
    fs::create_dir(&proj_dir)?;
    println!("project {} been made", &proj_dir);
    fs::create_dir(format!("{}/src", proj_dir))?; // chance to panic
    println!("{}/src been made", &proj_dir);
    fs::create_dir(format!("{}/build", &proj_dir))?;
    println!("{}/build has been made", &proj_dir);

    // create the files
    let mut src_file = fs::File::create(format!("{}/src/main.cpp", &proj_dir))?;
    let cpp_file = "#include <iostream>\n\nint main(){\n\tstd::cout << \"Hello, World!\\n\";\n\treturn 0;\n}";
    src_file.write_all(&cpp_file.as_bytes())?;

    let mut cmake_file = fs::File::create(format!("{}/CMakeLists.txt", &proj_dir))?;
    let cmake_src = format!(r#"
cmake_minimum_required(VERSION 3.12 FATAL_ERROR)
project({})
    
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

file(GLOB CXX_SOURCE_FILE \"./src/*.cpp\")
file(GLOB HEADER_FILE \"./src/*.hpp\")

add_executable($(PROJECT_NAME) $(CXX_SOURCE_FILE) $(HEADER_FILE))
    "#, &proj_dir);
    cmake_file.write_all(cmake_src.as_bytes())?;

    Ok(())
}

// gross
fn run_proj() {
    println!("run has not been implemented yet");
}

// gross
fn build_proj() {

}
