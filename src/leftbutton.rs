use crate::GameState;

pub fn left_button(state: &mut GameState, a: i32, b: i32) {
    let location = state.hero.y;
    match location {
    c if c == b => if state.hero.x > a-1  {
            state.hero.x -= 1;
        }, 
    c if c == b-1 => if state.hero.x > a-7 {
            state.hero.x -= 1;
        },
    c if c == b-2 => if state.hero.x > a+11 {
            state.hero.x -= 1;
        },
    c if c ==b-4 => if state.hero.x > a+6 {
            state.hero.x -= 1;
        }, 
    c if c == b+1 => state.hero.x -=1, 
    _ => {},
}
}
