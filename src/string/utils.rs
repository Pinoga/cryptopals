fn character_frequencies_scores_english() -> [u8; 128] {
    let mut table = [0u8; 128];

    // Populate the array with scores
    table[' ' as usize] = 27; // Space
    table['e' as usize] = 19;
    table['t' as usize] = 13;
    table['a' as usize] = 12;
    table['o' as usize] = 11;
    table['i' as usize] = 10;
    table['n' as usize] = 10;
    table['s' as usize] = 9;
    table['r' as usize] = 9;
    table['h' as usize] = 9;
    table['d' as usize] = 6;
    table['l' as usize] = 6;
    table['c' as usize] = 4;
    table['u' as usize] = 4;
    table['m' as usize] = 4;
    table['w' as usize] = 3;
    table['f' as usize] = 3;
    table['g' as usize] = 3;
    table['y' as usize] = 3;
    table['p' as usize] = 3;
    table['b' as usize] = 2;
    table['v' as usize] = 1;
    table['k' as usize] = 1;
    table['j' as usize] = 1;
    table['x' as usize] = 1;
    table['q' as usize] = 1;
    table['z' as usize] = 1;

    return table;
}

pub fn english_score(text: &[u8]) -> u32 {
    let scores: [u8; 128] = character_frequencies_scores_english();
    let mut score: u32 = 0;
    for char in text {
        score += scores[*char as usize] as u32;
    }
    return score as u32;
}
