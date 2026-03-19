pub fn assert_input_length(player_input: &str) -> bool {
    player_input.len() == 5
}

pub fn assert_input_alphabets_only(player_input: &str) -> bool {
    player_input.chars().all(|c| c.is_alphabetic())
}
