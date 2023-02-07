mod enums;
use std::cell::RefCell;

use enums::exploration::Exploration::UnExplored;
use enums::maze::Maze::Branch;
use enums::maze::Maze::Leaf;

fn main() {
    let leaf2 = Leaf(String::from("2"));
    let leaf4 = Leaf(String::from("4"));
    let leaf5 = Leaf(String::from("5"));
    let leaf8 = Leaf(String::from("8"));
    let branch3 = Branch(String::from("3"), &leaf4, &leaf5, RefCell::new(UnExplored));
    let branch1 = Branch(
        String::from("1"),
        &leaf2,
        &branch3,
        RefCell::new(UnExplored),
    );
    let branch7 = Branch(String::from("7"), &leaf5, &leaf8, RefCell::new(UnExplored));
    let branch6 = Branch(
        String::from("6"),
        &branch3,
        &branch7,
        RefCell::new(UnExplored),
    );
    let branch0 = Branch(
        String::from("0"),
        &branch1,
        &branch6,
        RefCell::new(UnExplored),
    );

    let mut buffer: Vec<String> = vec![];

    branch0.explore(&mut buffer);

    println!("{:?}", buffer)
}
