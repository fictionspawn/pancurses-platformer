use crate::GameState;

pub fn right_button(state: &mut GameState, a: i32, b: i32) {
    let hero = state.hero.y;
    match hero {
    c if c == b => if state.hero.x < a + 13  && state.hero.x > a-2 {
            state.hero.x += 1;
    },
    c if c == b-1 => if state.hero.x < a+13 {
            state.hero.x += 1;
        },
     c if c == b-2 => if state.hero.x < a+13 {
        state.hero.x += 1;     
    }, 
    c if c == b-4 => if state.hero.x < a+18 {
            state.hero.x += 1;
    },
    c if c == b+1 => if state.hero.x < a+6 {
        state.hero.x += 1;
    },
    _ => {},
}
}

