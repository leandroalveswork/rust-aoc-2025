use std::collections::HashSet;

use crate::mass_parser::read_lines;

pub struct Matrix {
    pub rows: Vec<Vec<bool>>,    
}

#[derive(Eq, Clone, Hash, PartialEq)]
pub struct Coordinate {
    pub x: i32, pub y: i32,
}

impl Matrix {
    pub fn get(&self, x: usize, y: usize) -> Option<&bool> {
        let row = self.rows.get(y)?;
        row.get(x)
    }

    pub fn has_roll(&self, x: i32, y: i32) -> bool {
        self.get(x as usize, y as usize)
            .map(|x| x.clone())
            .unwrap_or(false)
    }

    fn get_signed(&self, x: i32, y: i32) -> Option<&bool> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(x as usize, y as usize)
    }

    pub fn vertical_len(&self) -> usize {
        self.rows.len()
    }
    pub fn horizontal_len(&self) -> usize {
        self.rows.get(0).unwrap().len()
    }

    pub fn count_adjacents(&self, x: i32, y: i32) -> i8 {
        let adjacents: Vec<Option<&bool>> = vec![
            self.get_signed(x - 1, y - 1), self.get_signed(x, y - 1), self.get_signed(x + 1, y - 1),
            self.get_signed(x - 1, y    ),                            self.get_signed(x + 1, y    ),
            self.get_signed(x - 1, y + 1), self.get_signed(x, y + 1), self.get_signed(x + 1, y + 1),
        ];
        
        adjacents
            .iter()
            .filter(|x| x.map(|b| b.clone()) == Some(true))
            .count() as i8
    }

    fn _insert_changes(&self, changes: &mut HashSet<Coordinate>, coord: Coordinate) {
        let adjacents = vec![
            Coordinate { x: coord.x - 1, y: coord.y - 1 }, Coordinate { x: coord.x, y: coord.y - 1 }, Coordinate { x: coord.x + 1, y: coord.y - 1 },
            Coordinate { x: coord.x - 1, y: coord.y }, Coordinate { x: coord.x + 1, y: coord.y },
            Coordinate { x: coord.x - 1, y: coord.y + 1 }, Coordinate { x: coord.x, y: coord.y + 1 }, Coordinate { x: coord.x + 1, y: coord.y + 1 },
        ];

        let horiz = self.horizontal_len() as i32;
        let verti = self.vertical_len() as i32;

        for adjacent in adjacents {
            if adjacent.x <= -1 || adjacent.x >= horiz {
                continue;
            }
            if adjacent.y <= -1 || adjacent.y >= verti {
                continue;
            }
            if !changes.contains(&adjacent) {
                changes.insert(adjacent);
            }
        }
    }

    fn _remove_rolls(&mut self, changes: &mut HashSet<Coordinate>) {
        let changes_freeze = changes.clone();
        println!("Clearing {0} entries", changes.len());
        changes.clear();
        for change in changes_freeze {
            let x = change.x;
            let y = change.y;
            if !self.has_roll(x, y) || self.count_adjacents(x, y) >= 4 {
                continue;
            }

            self._insert_changes(changes, change);
            self.rows[y as usize][x as usize] = false;
        }
    }

    pub fn remove_rolls(&mut self) {
        let mut changes: HashSet<Coordinate> = HashSet::new();

        let verti = self.vertical_len() as i32;
        let horiz = self.horizontal_len() as i32;
        for y in 0..verti {
            for x in 0..horiz {
                if !self.has_roll(x, y) || self.count_adjacents(x, y) >= 4 {
                    continue;
                }

                self._insert_changes(&mut changes, Coordinate { x: x, y: y });
                self.rows[y as usize][x as usize] = false;
            }
        }

        while !changes.is_empty() {
            self._remove_rolls(&mut changes);
        }
    }
}

fn read_whole_matrix(lines: Vec<String>) -> Matrix {
    Matrix {
        rows: lines.iter()
            .map(|x|
                x.chars().map(|c| c == '@').collect())
            .collect()
    }
}

pub async fn answer() -> Option<()> {
    let lines = read_lines("day-04.txt").await?;
    // let lines = vec![
    //     "@@.@.@".to_string(),
    //     "@@.@@@".to_string(),
    //     "@@..@@".to_string(),
    // ];
    let matx = read_whole_matrix(lines);
    let horiz = matx.horizontal_len();
    let verti = matx.vertical_len();

    let easier_rolls: u32 = (0..(verti as i32))
        .map(|y| (0..(horiz as i32))
            .filter(|x| matx.has_roll(x.clone(), y) && matx.count_adjacents(x.clone(), y) < 4)
            .count() as u32)
        .sum();

    println!("{:?}", easier_rolls);

    Some(())
}

pub async fn answer2() -> Option<()> {
    let lines = read_lines("day-04.txt").await?;
    // let lines = vec![
    //     "@@.@.@".to_string(),
    //     "@@.@@@".to_string(),
    //     "@@..@@".to_string(),
    // ];
    let mut matx = read_whole_matrix(lines);
    
    let horiz = matx.horizontal_len();
    let verti = matx.vertical_len();
    let rolls_count: u32 = (0..(verti as i32))
        .map(|y| (0..(horiz as i32))
            .filter(|x| matx.has_roll(x.clone(), y))
            .count() as u32)
        .sum();

    matx.remove_rolls();

    let rolls_after_removal: u32 = (0..(verti as i32))
        .map(|y| (0..(horiz as i32))
            .filter(|x| matx.has_roll(x.clone(), y))
            .count() as u32)
        .sum();

    println!("{0} - {1} = {2}", rolls_count, rolls_after_removal, rolls_count - rolls_after_removal);

    Some(())
}
