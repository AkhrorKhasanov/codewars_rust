mod preloaded;
use preloaded::Player;

fn duck_duck_goose(players: &[Player], goose: u32) -> &'static str {
    let length: u32 = players.len() as u32;
    let mut hlp: u32 = (goose % length) as u32;
    if hlp == 0 {
        return &players[(length - 1) as usize].name;
    }
    &players[(hlp - 1) as usize].name
}