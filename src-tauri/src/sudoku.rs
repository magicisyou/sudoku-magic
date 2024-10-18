use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum CellType {
    Fixed(u8),
    Free(Option<u8>),
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Sudoku {
    pub values: [[CellType; 9]; 9],
    pub status: bool,
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            values: [[CellType::Free(None); 9]; 9],
            status: false,
        }
    }

    pub fn from_array(values: [[u8; 9]; 9]) -> Self {
        let mut result = Self::new();

        for (i, row) in values.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value == 0 {
                    result.values[i][j] = CellType::Free(None);
                } else {
                    result.values[i][j] = CellType::Fixed(value);
                }
            }
        }

        result
    }

    pub fn evaluate(&mut self) {
        let mut status = true;
        // Check rows
        for row in self.values {
            let sum: u8 = row
                .iter()
                .map(|cell| match cell {
                    CellType::Fixed(v) | CellType::Free(Some(v)) => *v,
                    CellType::Free(None) => 0,
                })
                .sum();
            if sum != 45 {
                status = false;
            }
        }

        // Check Columns
        for col in 0..9 {
            let sum: u8 = (0..9)
                .map(|row| match self.values[row][col] {
                    CellType::Fixed(v) | CellType::Free(Some(v)) => v,
                    CellType::Free(None) => 0,
                })
                .sum();
            if sum != 45 {
                status = false;
            }
        }

        // Check Squares
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut sum = 0;
                for row in 0..3 {
                    for col in 0..3 {
                        sum += match self.values[box_row * 3 + row][box_col * 3 + col] {
                            CellType::Fixed(v) | CellType::Free(Some(v)) => v,
                            CellType::Free(None) => 0,
                        };
                    }
                }
                if sum != 45 {
                    status = false;
                }
            }
        }

        self.status = status;
    }
}
