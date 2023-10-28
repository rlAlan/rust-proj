use std::env;


// proj new, run, build <project>

fn main() -> std::io::Result<()>{
    let mut cli_pattern = env::args().skip(1);
    match cli_pattern.len(){
        2 => { // len 2 new and potential future
            match cli_pattern.nth(0){
                Some(x) => {
                    if x == "new" {make_proj(cli_pattern.nth(0))?;}
                },None => println!("issue has happened")
            }
        },
        1 => {
            match cli_pattern.nth(0){
                Some(x) => {
                    if x == "build"{build_proj()}
                    if x == "run"{run_proj()}
                },None => println!("issue has happened")
            }
        }
        _ =>{println!("Not implemented yet");}
    }

    Ok(())
}

struct Project{
    name: String,
}


fn make_proj(command: Option<String>) -> std::io::Result<()>{
    use std::fs;
    let project = Project{name: command.expect("None")};
    fs::create_dir(format!("{}/", project.name))?;
    println!("project {} been made", project.name);
    Ok(())
}

fn run_proj(){
    println!("project has been run");
}

fn build_proj(){
    println!("project has been built");
}
