use rps_lib::types::RpsMoveType;

pub fn random_action() -> RpsMoveType {
    let random_i32 = rand::random_range(0..2);
    match random_i32 {
        0 => return RpsMoveType::Rock,
        1 => return RpsMoveType::Paper,
        2 => return RpsMoveType::Scissor,
        _ => return RpsMoveType::Scissor,
    }
    
}
