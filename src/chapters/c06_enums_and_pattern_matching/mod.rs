mod control_flow_match;
mod enums;
mod if_let;

pub fn run() {
    control_flow_match::main();
    enums::main();
    if_let::main();
}
