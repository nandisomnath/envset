use crate::core::get_shell;


mod core;





fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    let env_name = args.get(0).unwrap().clone();
    let env_value = args.get(0).unwrap().clone();

    let shell = get_shell();

    println!("{:?}", shell);
    


}
