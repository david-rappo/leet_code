#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 1 {
        return String::new();
    }

    let row_count = num_rows as usize;
    let bytes = s.as_bytes();
    let column_count = calculate_column_count(row_count, bytes.len());
    let mut grid = Grid::new(row_count, column_count);
    let mut is_vertical_traverse = true;
    let mut row_index = 0;
    let mut column_index = 0;
    let mut character_index = 0;
    let mut fill_grid = true;
    while fill_grid {
        if is_vertical_traverse {
            debug_assert_eq!(row_index, 0);
            while (row_index < row_count) && (character_index < bytes.len()) {
                let character = bytes[character_index];
                grid.set_value(row_index, column_index, character);
                row_index += 1;
                character_index += 1;
            }

            if character_index == bytes.len() {
                fill_grid = false;
            } else {
                is_vertical_traverse = false;
                if row_count > 1 {
                    row_index = row_count - 2;
                } else {
                    row_index = 0;
                }
                
                column_index += 1;
            }
        } else {
            while (character_index < bytes.len()) && (row_index > 0) {
                let character = bytes[character_index];
                grid.set_value(row_index, column_index, character);
                row_index -= 1;
                column_index += 1;
                character_index += 1;
            }

            if character_index == bytes.len() {
                fill_grid = false;
            } else {
                is_vertical_traverse = true;
                debug_assert_eq!(row_index, 0);
            }
        }
    }
    
    get_result(&grid, bytes.len())
}

struct Grid {
    row_count: usize,
    column_count: usize,
    elements: Vec<u8>
}

impl Grid {
    fn new(row_count: usize, column_count: usize) -> Grid {
        Grid { row_count, column_count, elements: vec![0; row_count * column_count] }
    }

    fn get_index(&self, row_index: usize, column_index: usize) -> usize {
        (row_index * self.column_count) + column_index
    }

    fn get_value(&self, row_index: usize, column_index: usize) -> u8 {
        let index = self.get_index(row_index, column_index);
        self.elements[index]
    }

    fn set_value(&mut self,
        row_index: usize,
        column_index: usize,
        value: u8) {
        let index = self.get_index(row_index, column_index);
        self.elements[index] = value;
    }
}

fn get_result(grid: &Grid, count: usize) -> String {
    let mut result = vec![0; count];
    let mut index = 0;
    for row_index in 0..grid.row_count {
        for column_index in 0..grid.column_count {
            let c = grid.get_value(row_index, column_index);
            if 0 != c {
                result[index] = c;
                index += 1;
            }
        }
    }
    
    String::from_utf8(result).unwrap()
}

fn calculate_column_count(row_count: usize, _character_count: usize) -> usize {
    // Example
    // row_count = 1
    // character_count = 14
    // 0 P A Y P A L I S H I R I N G

    // Example
    // row_count = 2
    // character_count = 14
    // 0 P Y A I H R N
    // 1 A P L S I I G

    // Example
    // row_count = 5
    // character_count = 14
    //
    // 0 P X X X H X
    // 1 A X X S I X
    // 2 Y X I X R X
    // 3 P L X X I G
    // 4 A X X X N X
    //
    // For every two vertical columns of characters there is row_count minus two
    // columns between them.
    
    // TODO:
    row_count
}