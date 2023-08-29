use std::{collections::HashSet, fmt::Debug};

const TILE_SIZE: usize = 10;
const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

#[derive(Clone)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<u8>>,
}

impl Tile {
    fn empty() -> Self {
        Self {
            id: 0,
            pixels: vec![vec![0; 10]; 10],
        }
    }

    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let tile_info = lines.next().unwrap();
        let id = tile_info[5..tile_info.len() - 1].parse().unwrap();

        Self {
            id,
            pixels: lines.map(|l| l.as_bytes().to_vec()).collect(),
        }
    }

    fn can_connect_below(&self, other: &Tile) -> bool {
        self.pixels.last() == other.pixels.first()
    }

    fn can_connect_right(&self, other: &Tile) -> bool {
        let size = self.pixels.len();
        for row in 0..size {
            if self.pixels[row][size - 1] != other.pixels[row][0] {
                return false;
            }
        }
        true
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.id)?;
        for row in &self.pixels {
            writeln!(f, "{}", std::str::from_utf8(row).unwrap())?;
        }
        Ok(())
    }
}

fn rotate(image: &mut Vec<Vec<u8>>) {
    let size = image.len();
    let mut rotated = image.clone();
    for (row, line) in rotated.iter_mut().enumerate() {
        for (col, pixel) in line.iter_mut().enumerate() {
            *pixel = image[col][size - 1 - row];
        }
    }
    *image = rotated
}

fn flip(image: &mut [Vec<u8>]) {
    for row in image.iter_mut() {
        row.reverse();
    }
}

struct Grid {
    layout: Vec<Vec<Tile>>,
    size: usize,
    solution: Option<Vec<Vec<Tile>>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.layout {
            for col in row {
                write!(f, "{:?}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(tiles: &[Tile]) -> Self {
        // 8 are the 4 possible rotations * 2 flips
        let size = ((tiles.len() / 8) as f64).sqrt() as usize;

        Self {
            layout: vec![vec![Tile::empty(); size]; size],
            size,
            solution: None,
        }
    }

    fn parse_tiles(input: &str) -> Vec<Tile> {
        let mut tiles = Vec::new();

        for tilemap in input.split("\n\n") {
            let mut tile = Tile::parse(tilemap);
            for _flips in 0..2 {
                for _rotations in 0..4 {
                    tiles.push(tile.clone());
                    rotate(&mut tile.pixels);
                }
                flip(&mut tile.pixels);
            }
        }
        tiles
    }

    fn search(&mut self, row: usize, col: usize, tiles: &[Tile], visited: &mut HashSet<usize>) {
        // Backtracking base case
        if row == self.size {
            self.solution = Some(self.layout.clone());
            return;
        }

        for tile in tiles {
            // Don't visit tiles with the same id, we already tried another rotation/flip
            if visited.contains(&tile.id) {
                continue;
            }

            // Tile validation
            if row > 0 && !self.layout[row - 1][col].can_connect_below(tile) {
                continue;
            }
            if col > 0 && !self.layout[row][col - 1].can_connect_right(tile) {
                continue;
            }

            self.layout[row][col] = tile.clone();
            visited.insert(tile.id);

            if col == self.size - 1 {
                self.search(row + 1, 0, tiles, visited);
            } else {
                self.search(row, col + 1, tiles, visited)
            }

            // Remove to backtrack
            visited.remove(&tile.id);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let tiles = Grid::parse_tiles(input);
    let mut grid = Grid::new(&tiles);

    grid.search(0, 0, &tiles, &mut HashSet::new());

    let Some(solution) = grid.solution else {
        return None;
    };

    let top_left = &solution[0][0];
    let top_right = &solution[0][grid.size - 1];
    let bottom_left = &solution[grid.size - 1][0];
    let bottom_right = &solution[grid.size - 1][grid.size - 1];

    let result = top_left.id * top_right.id * bottom_left.id * bottom_right.id;

    Some(result)
}

fn assemble_picture(tilemap: &[Vec<Tile>], grid_size: usize) -> Vec<Vec<u8>> {
    let mut picture = Vec::new();
    let total_grid_size = grid_size * TILE_SIZE;
    for row in 0..total_grid_size {
        if row % TILE_SIZE == 0 || row % TILE_SIZE == TILE_SIZE - 1 {
            continue;
        }
        let mut full_row = Vec::new();
        for col in 0..total_grid_size {
            if col % TILE_SIZE == 0 || col % TILE_SIZE == TILE_SIZE - 1 {
                continue;
            }
            let pixel =
                tilemap[row / TILE_SIZE][col / TILE_SIZE].pixels[row % TILE_SIZE][col % TILE_SIZE];
            full_row.push(pixel);
        }
        picture.push(full_row);
    }

    picture
}

fn count_monsters(picture: &mut Vec<Vec<u8>>) {
    for _flips in 0..2 {
        for _rotations in 0..4 {
            for row in 0..(picture.len() - MONSTER.len() + 1) {
                for col in 0..(picture[0].len() - MONSTER[0].len() + 1) {
                    try_marking_monster(picture, row, col);
                }
            }
            rotate(picture);
        }
        flip(picture);
    }
}

fn try_marking_monster(picture: &mut [Vec<u8>], start_row: usize, start_col: usize) -> bool {
    for (monster_row, monster_line) in MONSTER.iter().enumerate() {
        for (monster_col, monster_char) in monster_line.bytes().enumerate() {
            let tile_char = picture[start_row + monster_row][start_col + monster_col];
            if monster_char == b'#' && tile_char != b'#' {
                return false;
            }
        }
    }

    for (monster_row, monster_line) in MONSTER.iter().enumerate() {
        for (monster_col, monster_char) in monster_line.bytes().enumerate() {
            if monster_char == b'#' {
                picture[start_row + monster_row][start_col + monster_col] = b'O';
            }
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<usize> {
    let tiles = Grid::parse_tiles(input);
    let mut grid = Grid::new(&tiles);

    grid.search(0, 0, &tiles, &mut HashSet::new());

    let Some(solution) = grid.solution else {
        return None;
    };

    let mut picture = assemble_picture(&solution, grid.size);
    count_monsters(&mut picture);
    // for row in &picture {
    //     println!("{}", std::str::from_utf8(row).unwrap());
    // }

    let sea_harshness = picture.iter().flatten().filter(|&&b| b == b'#').count();

    Some(sea_harshness)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(20899048083289));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(273));
    }
}
