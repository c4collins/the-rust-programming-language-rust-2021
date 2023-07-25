mod control_flow;
mod functions;
mod types;
mod variables;

pub fn run() {
    println!("Running Chapter 3");
    control_flow::main();
    functions::main();
    types::main();
    variables::main();
}
