//width: 40, height: 20, size: 800;
//positions as u16, 0x00ff as x, 0xff00 as y
//pos 0 in bottom left
//x-> y V

use inputbot::KeybdKey::{self, *};
use rand::prelude::*;

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

fn move_apple(snake : Vec<u16>, apple : &mut u16){
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut pos: u16 = get_pos(rng.gen_range(0..40), rng.gen_range(0..20));
    while snake.contains(&pos) {
        pos = get_pos(rng.gen_range(0..40), rng.gen_range(0..20));
    }
    *apple = pos;
}

fn add_dir(pos: u16, dir: u8) -> u16 {
    match dir {
        0 => pos-0x100,
        1 => pos+1,
        2 => pos+0x100,
        3 => pos-1,
        4.. =>pos,
    }
}

//return true if snake is still alive
//return false if snake is dead
fn move_snake(snake : &mut Vec<u16>, head_dir: u8, apple : &mut u16) -> bool {
    let headpos: u16 = *snake.last().unwrap();
    let newheadpos: u16 = add_dir(headpos, head_dir);
    if snake.contains(&newheadpos) || get_x(newheadpos) >= 40 || get_y(newheadpos) >= 20{
        return false;
    }
    snake.push(newheadpos);
    if newheadpos != *apple {
        snake.remove(0);
        return true;
    }
    move_apple(snake.to_vec(), apple);
    return true;
}

fn main() {
    let mut apple: u16 = get_pos(10, 10);
    let mut snake: Vec<u16> = Vec::new();
    let tick = 1000;
    // 0 -> up
    // 1 -> right
    // 2 -> down
    // 3 -> left
    let mut head_dir: u8 = 1;
    let movement_keys : [KeybdKey; 4] = [WKey, DKey, SKey, AKey];
    let exit_key : KeybdKey = EscapeKey;
    for x in 3..6 {
        snake.push(get_pos(x, 10));
    }
    loop {
        if exit_key.is_pressed(){ 
            break;
        }
        let head_pos: u16 = *snake.last().unwrap();
        for key in 0..4 {
            if movement_keys[key].is_pressed() &! snake.contains(&add_dir(head_pos, key as u8)){
                head_dir = key as u8;
            }
        }
        if !move_snake(&mut snake, head_dir, &mut apple) {
            break;
        }
        print_state(apple, snake.clone());
        std::thread::sleep(std::time::Duration::from_millis(tick));
    }
}