fn has_a_mine(field: char) -> bool {
    field == '*'
}

fn find_the_nof_mines(board: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut num_mines = 0;
    // Check top
    if y > 0 {
        if has_a_mine(board[y-1][x]){
            num_mines += 1;
        }
        // Check top left
        if x > 0 {
            if has_a_mine(board[y-1][x-1]){
                num_mines += 1;
            }
        }

        // Check top right
        if x < board[y].len() -1 {
            if has_a_mine(board[y-1][x+1]){
                num_mines +=1;
            }
        }
    }
    // Check bottom
    if y < board.len() -1 {
        if has_a_mine(board[y+1][x]){
            num_mines += 1;
        }

        // Check bottom left
        if x > 0 {
            if has_a_mine(board[y+1][x-1]){
                num_mines += 1;
            }
        }

        // Check bottom right
        if x < board[y].len() -1 {
            if has_a_mine(board[y+1][x+1]){
                num_mines +=1;
            }
        }
    }
    // Check left
    if x > 0 {
        if has_a_mine(board[y][x-1]){
            num_mines += 1;
        }
    }
    // Check right
    if x < board[y].len() -1 {
        if has_a_mine(board[y][x+1]){
            num_mines +=1;
        }
    }
    return num_mines;
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board: Vec<Vec<char>> = minefield.iter().map(|c| c.to_string().chars().collect()).collect();
    let mut result: Vec<String> = vec![];
    // Go through the minefield and count the nof mines 
    for y in 0..board.len() {
        let mut current_row = String::from("");
        for x in 0..board[y].len() {
            if !has_a_mine(board[y][x]){
                let num_mines = find_the_nof_mines(&board, y, x);
                if num_mines > 0{
                    assert!(num_mines > 0);
                    assert!(num_mines < 9);
                    let mine_char = (num_mines as u8 + b'0') as char;
                    current_row.push(mine_char);
                } else {
                    current_row.push(' ');
                }
            } else {
               current_row.push('*');
            }
        }
        result.push(current_row);
    }
    result
}
