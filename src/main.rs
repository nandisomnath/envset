use crate::core::add_env;

mod core;



fn usage() {

}



fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    let env_name = args.get(0).unwrap().clone();
    let env_value = args.get(0).unwrap().clone();


    add_env(env_name, env_value);


}
