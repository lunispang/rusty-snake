//width: 40, height: 20, size: 800;
//positions as u16, 0x00ff as x, 0xff00 as y
//pos 0 in bottom left
//x-> y ^

fn get_pos(x: u8, y: u8) -> u16 {
    return (x as u16) + (y as u16) << 8;
}

fn main() {
    let mut out: [u8; 800];
    let mut apple: u16 = get_pos(10, 20);
    let mut snake: Vec<u16> = Vec::new();
    // 0 -> up
    // 1 -> right
    // 2 -> down
    // 3 -> left
    let mut head_dir: u8 = 1;
    for x in 3..6 {
        snake.push(get_pos(x, 20));
    }
}