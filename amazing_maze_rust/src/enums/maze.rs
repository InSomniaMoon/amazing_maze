// 1. Définir la structure d'un labyrinthe et d'implémenter l'exemple

use std::cell::RefCell;

use Exploration::*;

use crate::enums::maze::Maze::Branch;
use crate::enums::maze::Maze::Leaf;

use super::exploration::Exploration;

pub enum Maze<'a> {
    Branch(String, &'a Maze<'a>, &'a Maze<'a>, RefCell<Exploration>),
    Leaf(String),
}

impl Maze<'_> {
    pub fn explore(&self, buffer: &mut Vec<String>) {
        match self {
            Branch(label, left, right, statut) => {
                if *statut.borrow() == UnExplored {
                    statut.replace(Explored);
                    buffer.push(label.to_string());
                    left.explore(buffer);
                    right.explore(buffer);
                } else {
                    buffer.push(label.to_string());
                }
            }
            Leaf(label) => buffer.push(label.to_string()),
        }
    }
}
