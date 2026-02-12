use crate::GameState;
use pancurses::Window; 

const HEAD_SYMBOL: char = 'O';
const WORMMAN: char = 'S';
const LADDER_UP: char = 'H';
const LADDER_DOWN: char = ' ';
const AIR: char = ' ';
const LAMP: char = '+';
const CHAIN: char = '|';
const CHAIN_LEFT: char = '/';
const CHAIN_RIGHT: char = '\\';
const WINDOW: char = 'I';
const FLOOR_TILE: char = '_';
const WALL: char = '|';
const DOOR: char = 'D';
const LEFT_ROOF: char = '/';
const RIGHT_ROOF: char = '\\';

pub fn build_scene(state: &mut GameState, window: &Window, a: i32, b: i32) {
     //Draw the currently visible stage
      window.mv(b-8, 1);
      window.printw("Welcome to Wormmen Invasion!");
      window.refresh();
      window.mv(b-7, 1);
      window.refresh();
      window.printw(&format!("{}   ", state.hero.x - 86));
      window.mv(b-7, 6);
      window.printw(&format!("{}   ", (state.hero.y - 20) * -1));
      window.mv(b-5, 1);
      //window.printw(&format!("{}, {}", a, b));
      window.refresh();
        if state.hero.y == b && state.hero.x > a-2 {
            window.mv(b-6, 1);
            window.printw("You are in a dark cellar of some sorts.     ");
            window.mv(b-5, 1);
            window.printw("There's a ladder some steps to the right,   ");
            window.mv(b-4, 1);
            window.printw("and a door further down the hall.           ");
            window.mv(b-3, 1);
            window.printw("                                             ");
            window.mv(state.hero.y, a-1);
            window.hline(FLOOR_TILE, 15);
            if state.wormman.x == a+5 {
                window.mvaddch(state.wormman.y, state.wormman.x, LADDER_DOWN);
            } else if state.wormman.x == a-5 {
                window.mvaddch(state.wormman.y, state.wormman.x, DOOR);
            } else {
            window.mvaddch(state.wormman.y, state.wormman.x, FLOOR_TILE);
            }
        } else if state.hero.y == b-1 || state.hero.y == b-2 || state.hero.y == b-3 || state.hero.y == b-4 {
            if state.hero.y == b-1 {
                window.mv(b-6, 1);
                window.printw("A disgusting creature, half man, half worm,  ");
                window.mv(b-5, 1);
                window.printw("is crawling around up here. There's a lamp   ");
                window.mv(b-4, 1);
                window.printw("hanging from the ceiling to your right.       ");
                window.mv(b-3, 1);
                window.printw("A door is seen to the left.                 ");
            }
            if state.hero.y == b-2 && state.hero.x == a+12 {
                window.mv(b-6, 1);
                window.printw("You hang on to the lamp. There's a window   ");
                window.mv(b-5, 1);
                window.printw("To your right. The worrman is drewling     ");
                window.mv(b-4, 1);
                window.printw("back and forth below.                      ");
                window.mv(b-3, 1);
                window.printw("                                             ");
            }
            window.mv(b-1, a - 7);
            window.hline(FLOOR_TILE, 21);
            window.mvaddch(b-1, a+14, WALL);
            window.mvaddch(b-1, a-8, WALL);
            window.mvaddch(b-2, a+12, LAMP); 
            window.mvaddch(b-1, a-5, DOOR);
            if state.hero.y == b-2 {
                if state.hero.x == a+11 || state.hero.x == a+13 {
            window.mvaddch(b-2, a+12, AIR);
                } 
            }
            window.mvaddch(b-3, a+12, CHAIN);
            if state.hero.y == b-2 {
                if state.hero.x == a+11 {
                    window.mvaddch(b-3, a+12, CHAIN_LEFT);
                 } else if state.hero.x ==a+13 {
                    window.mvaddch(b-3, a+12, CHAIN_RIGHT);
                 }
            }
            if !state.reqs.window_broken {
                window.mvaddch(b-2, a+14, WINDOW);
            } else {
                window.mvaddch(b-2, a+14, AIR);
            }
            window.mvaddch(b-3, a+14, WALL);
            window.mvaddch(b-1, a+5, LADDER_DOWN);
            window.mvaddch(state.wormman.y, state.wormman.x, WORMMAN);
        } else if state.hero.y == b+1 {
            window.mv(b+1, a-15);
            window.hline(FLOOR_TILE, 23);
            window.mvaddch(b+1, a+7, WALL);
            window.mvaddch(b+1, a+2, AIR);
        }
        if state.hero.y == b-1 && state.hero.x == a-5 {
            window.mvaddch(b, a-4, FLOOR_TILE);
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
}


