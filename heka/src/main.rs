use std::io;

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
            "navbar" => println!("navbar"),
            "footer" => println!("footer"),
            _ => println!("fuck wrong input"),
        };
    }
}
// /home/nilay/Projects/WebsiteComponents/NavBar
