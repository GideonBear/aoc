use std::fs;

fn main() {
    let text = fs::read_to_string("19.txt").expect("Error while reading file");

    let (workflows, parts) = text.split("\n\n");
    let workflows = workflows.map(Workflow::from_string);
    let parts = parts.map(Part::from_string);
}
