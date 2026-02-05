use crate::GameState;

pub fn down_button(state: &mut GameState, a: i32, b: i32) {
    if state.hero.y == b-1 {
        if state.hero.x == a+5 {
            state.hero.y += 1;
        } else if state.hero.x == a-5 {
            state.hero.y += 1;
            state.hero.x += 1;
        }
    } else if state.hero.y == b-2 || state.hero.y == b-3 || state.hero.y == b-4 {
        if state.hero.x == a+12 {
            state.hero.y += 1;
        }
    } else if state.hero.y == b {
        if state.hero.x == a-4 {
            state.hero.y +=1;
            state.hero.x +=1;
        }
    }
}
