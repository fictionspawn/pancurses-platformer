extern crate pancurses;
use pancurses::{initscr, endwin, Input, noecho, curs_set};
use std::time::{Duration, SystemTime};

mod upbutton;
use crate::upbutton::up_button;
mod downbutton;
use crate::downbutton::down_button;
mod leftbutton;
use crate::leftbutton::left_button;
mod rightbutton;
use crate::rightbutton::right_button;
mod buildscene;
use crate::buildscene::build_scene;

const HEAD_SYMBOL: char = 'O';
const AIR: char = ' ';
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

pub struct Reqs {
    lamp_l: bool,
    lamp_r: bool,
    window_broken: bool,
}

pub struct GameState {
    pub hero: Hero,
    pub wormman: Wormman,
    pub reqs: Reqs,
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
            reqs: Reqs {
                lamp_l: false,
                lamp_r: false,
                window_broken: false,
            },
        }
    }
}

pub fn game() {
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
    window.nodelay(true);

    let mut wormman_left = true;
    
    let mut last_wormman_update = SystemTime::now();
    let wormman_interval = Duration::from_millis(500);

    //Game loop
    loop {
        build_scene(&mut state, &window, a, b);

        if state.wormman.y == state.hero.y {
            if wormman_left {
                if state.wormman.x == state.hero.x || state.wormman.x == state.hero.x  {
                    break 
                }
            } else {
                if state.wormman.x == state.hero.x || state.wormman.x == state.hero.x  {
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

        //Control wormmen and other time based stage movements
        if last_wormman_update.elapsed().unwrap_or(Duration::from_secs(0)) >= wormman_interval {
            if wormman_left {
               state.wormman.x -= 1;
            } else {
                state.wormman.x += 1;
            }
            if state.hero.y == b-2 {
                if state.hero.x >= a+11 && state.hero.x <= a+13 {
                    if state.hero.x == a+13 {
                    if state.reqs.lamp_l == false {
                        state.reqs.lamp_r = true;
                        state.hero.x = a+12;
            window.mvaddch(b-2, a+11, AIR);
            window.mvaddch(b-2, a+13, AIR);
                    } else {
                        state.hero.x = a+16;
                        state.reqs.window_broken = true;
                       // state.hero.story_text = "You jump off the lamp and through the window in the wall. You fall on the floor in a dark room.".to_string(); 
                        state.reqs.lamp_r = false;
                        state.reqs.lamp_l = false;
                    }
            window.mvaddch(b-2, a+11, AIR);
            window.mvaddch(b-2, a+13, AIR);
                    }
                }
                if state.hero.x == a+11 {
                    if state.reqs.lamp_r {
                        state.reqs.lamp_l= true;
                        state.hero.x = a+12;
            window.mvaddch(b-2, a+13, AIR);
            window.mvaddch(b-2, a+11, AIR);
                    } else {
                        state.hero.x = a+12;
            window.mvaddch(b-2, a+11, AIR);
            window.mvaddch(b-2, a+13, AIR);
                    }
                }
            }
            last_wormman_update = SystemTime::now();
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
                    c if c == UP_KEY => up_button(&mut state, a, b),

                    c if c == DOWN_KEY => down_button(&mut state, a, b),
                       
                    c if c == LEFT_KEY => left_button(&mut state, a, b),                     
                    c if c == RIGHT_KEY  => right_button(&mut state, a, b),                    _ => {} 
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
            Some(_input) => {
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


