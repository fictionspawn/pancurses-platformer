use crate::GameState;

pub fn left_button(state: &mut GameState, a: i32, b: i32) {
    if state.hero.y == b { 
        if state.hero.x > a-1  {
            state.hero.x -= 1;
        } 
    } else if state.hero.y ==  b-1 { 
        if state.hero.x > a-7 {
            state.hero.x -= 1;
        }
    } else if state.hero.y == b-2 {
        if state.hero.x > a+11 {
            state.hero.x -= 1;
        }
    } else if state.hero.y == b-4 {
        if state.hero.x > a+6 {
            state.hero.x -= 1;
        } 
    } else if state.hero.y ==  b+1 {
        state.hero.x -=1
    }
}
