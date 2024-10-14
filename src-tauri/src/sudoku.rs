use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Sudoku {
    pub values: [[u32; 9]; 9],
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            values: [[0; 9]; 9],
        }
    }

    pub fn is_completed(&self) -> bool {
        // Check rows
        for row in self.values {
            let sum: u32 = row.iter().sum();
            if sum != 45 {
                return false;
            }
        }

        // Check Columns
        for col in 0..=8 {
            let mut sum = 0;
            for row in 0..=8 {
                sum += self.values[row][col];
            }
            if sum != 45 {
                return false;
            }
        }

        // Check Squares
        let mut box_sum = [0; 9];
        for row in 0..=8 {
            for col in 0..=8 {
                if row < 3 {
                    if col < 3 {
                        box_sum[0] += self.values[row][col];
                    } else if col < 6 {
                        box_sum[1] += self.values[row][col];
                    } else {
                        box_sum[2] += self.values[row][col];
                    }
                } else if row < 6 {
                    if col < 3 {
                        box_sum[3] += self.values[row][col];
                    } else if col < 6 {
                        box_sum[4] += self.values[row][col];
                    } else {
                        box_sum[5] += self.values[row][col];
                    }
                } else if col < 3 {
                    box_sum[6] += self.values[row][col];
                } else if col < 6 {
                    box_sum[7] += self.values[row][col];
                } else {
                    box_sum[8] += self.values[row][col];
                }
            }
        }
        for val in box_sum {
            if val != 45 {
                return false;
            }
        }
        true
    }
}
