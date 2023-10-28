use std::env;

// proj new, run, build <project>

fn main() -> std::io::Result<()>{
    // ignore progarm name 
    let mut cli_pattern = env::args().skip(1);
    // match by size of command line input
    match cli_pattern.len(){
        2 => { // len 2 new and potential future
            // idk why nth(0) twice i think it increments the iterator
            match cli_pattern.nth(0){
                // take the stuffing out the chicken
                Some(x) => {
                    if x == "new" {make_proj(cli_pattern.nth(0))?;}
                },None => println!("issue has happened")
                // if no stuffing
            }
        },
        1 => {
            match cli_pattern.nth(0){
                Some(x) => {
                    // make it ugly use more match
                    if x == "build"{build_proj()}
                    if x == "run"{run_proj()}
                },None => println!("issue has happened")
            }
        }
        // not implemented you terrible coder
        _ =>{println!("Not implemented yet");}
    }
    // to be safe for that one random std::io::Result<()>
    Ok(())
}

// going to store more stuff than just project name maybe 
// like folders inside the main project one
struct Project{
    name: String,
}

// dont know if i should add as like a trait or crate or something
// for the Project class
// idk 
// reminds me of more complex C but stfu
fn make_proj(command: Option<String>) -> std::io::Result<()>{
    use std::fs;
    let project = Project{name: command.expect("None")};
    // looks like filesystem in cpp17 -such a slay
    fs::create_dir(format!("{}/", project.name))?;
    println!("project {} been made", project.name);
    Ok(())
}

// gross
fn run_proj(){
    println!("project has been run");
}

// gross
fn build_proj(){
    println!("project has been built");
}
