use std::io;
use std::process::Command; // for executing command
fn add_component(component_name: &str, project_name: &str) {
    let path: String = String::from("/home/nilay/Projects/WebsiteComponents/") + component_name;
    let projectpath: String = String::from("./") + project_name + "/src/" + component_name;
    println!("project path - {}", projectpath);
    println!("component path - {} ", path);
    // cp <source> <destination>
    Command::new("cp")
        .arg("-r")
        .arg(path)
        .arg(projectpath)
        .output()
        .expect("Can't copy the file");
}

fn main() {
    println!("Enter the components :");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    let components: Vec<&str> = input.split(' ').collect();

    for i in &components {
        let component: &str = i;
        match component.trim() {
            "navbar" => add_component("NavBar", components[0]),
            "footer" => add_component("footer", components[0]),
            _ => println!("fuck wrong input"),
        };
    }
}
// /home/nilay/Projects/WebsiteComponents/NavBar
