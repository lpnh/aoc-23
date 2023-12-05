use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vertex(pub usize, pub usize);

#[derive(Clone)]
pub struct Graph {
    pub grid: Vec<Vec<char>>,
}

impl Graph {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        Graph { grid }
    }

    pub fn neighbors(&self, vertex: Vertex) -> Vec<Vertex> {
        let mut neighbors = Vec::new();
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), /* (0, 0), */ (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let new_x = vertex.0 as i32 + dx;
            let new_y = vertex.1 as i32 + dy;
            if new_x >= 0 && new_x < self.grid.len() as i32 &&
               new_y >= 0 && new_y < self.grid[0].len() as i32 {
                neighbors.push(Vertex(new_x as usize, new_y as usize));
            }
        }
        neighbors
    }

    pub fn dfs(&self, vertex: Vertex, visited: &mut HashSet<Vertex>, number: &mut i32, include_number: &mut bool) {
        if visited.insert(vertex) {
            if let Some(digit) = self.grid[vertex.0][vertex.1].to_digit(10) {
                *number = *number * 10 + digit as i32;
                for neighbor in self.neighbors(vertex) {
                    let neighbor_cell = self.grid[neighbor.0][neighbor.1];
                    if neighbor_cell.is_ascii_digit() {
                        self.dfs(neighbor, visited, number, include_number);
                    } else if neighbor_cell != '.' {
                        *include_number = true;
                    }
                }
            }
        }
    }
}
