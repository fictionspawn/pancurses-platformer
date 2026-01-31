extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, curs_set};

const HEAD_SYMBOL: char = 'O';
const WORMMAN: char = 'S';
const LADDER_UP: char = 'H';
const LADDER_DOWN: char = ' ';
const LAMP: char = '+';
const CHAIN: char = '|';
const CHAIN_LEFT: char = '/';
const CHAIN_RIGHT: char = '\\';
const WINDOW: char = 'I';
const FLOOR_TILE: char = '_';
const WALL: char = '|';
const LEFT_ROOF: char = '/';
const RIGHT_ROOF: char = '\\';
const UP_KEY: char = 'w';
const DOWN_KEY: char = 's';
const LEFT_KEY: char = 'a';
const RIGHT_KEY: char = 'd';

pub struct Hero {
    x: i32,
    y: i32,
}

pub struct Wormman {
    x: i32,
    y: i32,
}

pub struct GameState {
    pub hero: Hero,
    pub wormman: Wormman,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            hero: Hero {
                x: 0,
                y: 0,
            },
            wormman: Wormman {
                x: 10,
                y: 19,
            },
        }
    }
}

fn main() {
    let mut state = GameState::new();
    let window = initscr();
    window.keypad(true);
    noecho();
    curs_set(0); // Hide the cursor
    
    // Get initial screen dimensions
    let (mut max_y, mut max_x) = window.get_max_yx();
    //let (mut min_y, mut min_x) = window.get_min_xy();
    let a = max_x / 2;
    let b = max_y / 2;
    
    // Start position (center of screen)
    state.hero.x = a;
    state.hero.y = b;

    state.wormman.x = a+10;
    state.wormman.y = b-1;
    // Draw initial position
    window.mvaddch(state.hero.y, state.hero.x, HEAD_SYMBOL);
    window.refresh();
    
   /* let i = 0;
    loop {
        window.mvaddch(b, i, FLOOR_TILE); 
        if i == max_x {
            break
        }
   window.mvaddch(b-2, a+14, WALL);
   window.mvaddch(b-3, a+14, WALL);
    }*/


            //thread::sleep(Duration::from_millis(difficulty.get_refresh_delay()));
    let mut wormman_left = true;

    loop {
        if state.hero.y == b {
            window.mv(state.hero.y, a-1);
            window.hline(FLOOR_TILE, 15);
            if !wormman_left {
            if state.wormman.x == a+6 {
                window.mvaddch(state.wormman.y, state.wormman.x, LADDER_DOWN);
            } else {
            window.mvaddch(state.wormman.y, state.wormman.x -1, FLOOR_TILE);
            }
            } else {
                if state.wormman.x == a+4 {
                window.mvaddch(state.wormman.y, state.wormman.x, LADDER_DOWN);
            } else {
                window.mvaddch(state.wormman.y, state.wormman.x +1, FLOOR_TILE);
                }
            }
        } else if state.hero.y == b-1 || state.hero.y == b-2 || state.hero.y == b-3 || state.hero.y == b-4 {
            window.mv(b-1, a - 7);
            window.hline(FLOOR_TILE, 21);
            window.mvaddch(b-1, a+14, WALL);
            window.mvaddch(b-1, a-8, WALL);
            window.mvaddch(b-2, a+12, LAMP);
            window.mvaddch(b-3, a+12, CHAIN);
            window.mvaddch(b-2, a+14, WINDOW);
            window.mvaddch(b-3, a+14, WALL);
            window.mvaddch(b-1, a+5, LADDER_DOWN);
            window.mvaddch(state.wormman.y, state.wormman.x, WORMMAN);
        }
        window.mv(state.hero.y, state.hero.x);
        window.mvaddch(b, a + 5, LADDER_UP);
        if state.hero.y == b - 4 {
            window.mv(state.hero.y, a+6);
            window.hline(FLOOR_TILE, 13);
            window.mvaddch(b-4, a+12, LADDER_DOWN);
            window.mvaddch(b-4, a+5, LEFT_ROOF);
            window.mvaddch(b-4, a+19, RIGHT_ROOF);
        }
        window.mvaddch(b, a-2, WALL);
        window.mvaddch(b, a+14, WALL);
        window.mvaddch(state.hero.y, state.hero.x, HEAD_SYMBOL);
        if state.wormman.y == state.hero.y {
            if wormman_left {
                if state.wormman.x == state.hero.x || state.wormman.x == state.hero.x - 1 {
                    break 
                }
            } else {
                if state.wormman.x == state.hero.x || state.wormman.x == state.hero.x + 1 {
            break
        }
            }
        }
        if state.wormman.x < a-6 {
            wormman_left = false;
        }
        if state.wormman.x > a+12 {
            wormman_left = true;
        }
        if wormman_left {
           state.wormman.x -= 1;
        } else {
            state.wormman.x += 1;
        }

        // Handle window resize
        let (new_max_y, new_max_x) = window.get_max_yx();
        if new_max_y != max_y || new_max_x != max_x {
            max_y = new_max_y;
            max_x = new_max_x;
            // Keep character within bounds if screen shrinks
            if state.hero.x >= max_x { state.hero.x = max_x - 1; }
            if state.hero.y >= max_y { state.hero.y = max_y - 1; }
            if state.hero.x < 0 { state.hero.x = 0; }
            if state.hero.y < 0 { state.hero.y = 0; }
            window.clear();
        }
        
        
        match window.getch() {
            Some(Input::Character(c)) => {
                let c_lower = c.to_ascii_lowercase();
                
                // Clear current position
                window.mvaddch(state.hero.y, state.hero.x, ' ');
                
                // Update position based on keyi and position
                match c_lower {
                    c if (c == UP_KEY && state.hero.y == b && state.hero.x == a+5) || (c == UP_KEY && state.hero.y == b-1 && state.hero.x == a+12) || (c == UP_KEY && state.hero.y == b-2 && state.hero.x == a+12) || (c == UP_KEY && state.hero.y == b-3 && state.hero.x == a+12) => state.hero.y -= 1,
                    c if (c == DOWN_KEY && state.hero.y == b-1 && state.hero.x == a+5) || (c == DOWN_KEY && state.hero.y == b-2 && state.hero.x == a+12) || (c == DOWN_KEY && state.hero.y == b-3 && state.hero.x == a+12) || (c == DOWN_KEY && state.hero.y == b-4 && state.hero.x == a+12) => state.hero.y += 1,
                    c if c == LEFT_KEY => if state.hero.y == b { 
                        if state.hero.x > a-1  {
                            state.hero.x -= 1;
                        } 
                    } else if state.hero.y ==  b-1 { 
                        if state.hero.x > a-7 {
                        state.hero.x -= 1;
                        }
                    } else if state.hero.y == b-4 {
                        if state.hero.x > a+6 {
                            state.hero.x -= 1;
                        }
                    },
                    c if c == RIGHT_KEY  => if state.hero.y == b || state.hero.y == b - 1 { 
                        if state.hero.x < a + 13 {
                            state.hero.x += 1;
                        }
                    } else if state.hero.y == b-4 {
                        if state.hero.x < a+18 {
                            state.hero.x += 1;
                        }
                    },
                    _ => {} 
                }
         
                // Draw character at new position
                window.mvaddch(state.hero.y, state.hero.x, HEAD_SYMBOL);
                window.refresh();
            },
            
            Some(Input::KeyDC) => break, // Delete key to exit
            Some(Input::KeyResize) => {
                // Handle resize immediately
                window.clear();
                window.refresh();
            },
            Some(input) => {
                // Optional: Show what key was pressed (for debugging)
                // window.mvprintw(0, 0, &format!("Key: {:?}    ", input));
                // window.mvaddch(y, x, HEAD_SYMBOL);
                // window.refresh();
            },
            None => ()
        }
    }
    endwin();
}
