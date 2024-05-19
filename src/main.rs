//width: 40, height: 20, size: 800;
//positions as u16, 0x00ff as x, 0xff00 as y
//pos 0 in bottom left
//x-> y ^

fn get_pos(x: u8, y: u8) -> u16 {
    return (x as u16) + ((y as u16) << 8);
}

fn get_x(pos: u16) -> u8 {
    return (pos & 0xff) as u8;
}

fn get_y(pos: u16) -> u8 {
    return ((pos & 0xff00) >> 8) as u8;
}

fn print_state(apple : u16, snake : Vec<u16>) {
    let mut current_line : [char; 40];
    let mut current_str : String;
    let apple_y: u8 = get_y(apple);
    for y in 0..20 {
        current_line = [' '; 40];
        if apple_y == y {
            current_line[get_x(apple) as usize] = 'O';
        }
        for x in 0..40 {
            if snake.contains(&get_pos(x, y)) {
                current_line[x as usize] = '#';
            }
        }
        current_str = current_line.iter().cloned().collect();
        println!("{}",  current_str);
    }
}

fn main() {
    let mut apple: u16 = get_pos(10, 10);
    let mut snake: Vec<u16> = Vec::new();
    // 0 -> up
    // 1 -> right
    // 2 -> down
    // 3 -> left
    let mut head_dir: u8 = 1;
    for x in 3..6 {
        snake.push(get_pos(x, 10));
    }

    print_state(apple, snake)
}