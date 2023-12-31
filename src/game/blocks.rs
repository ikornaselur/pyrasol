/// Which card indexes the card directly blocks
/// Returns a tuple of the lowest index blocked and how many cards are blocked, that is the value
/// (3, 1) means that index 3 is blocked while (3, 2) means index 3 and 4 are blocked
pub fn card_directly_blocks(card: usize) -> (usize, usize) {
    match card {
        0 => (0, 0),
        1 | 2 => (0, 1),
        3 => (1, 1),
        4 => (1, 2),
        5 => (2, 1),
        6 => (3, 1),
        7 => (3, 2),
        8 => (4, 2),
        9 => (5, 1),
        10 => (6, 1),
        11 => (6, 2),
        12 => (7, 2),
        13 => (8, 2),
        14 => (9, 1),
        15 => (10, 1),
        16 => (10, 2),
        17 => (11, 2),
        18 => (12, 2),
        19 => (13, 2),
        20 => (14, 1),
        21 => (15, 1),
        22 => (15, 2),
        23 => (16, 2),
        24 => (17, 2),
        25 => (18, 2),
        26 => (19, 2),
        27 => (20, 1),
        _ => (0, 0),
    }
}

pub fn card_blocks(card: usize) -> Vec<usize> {
    match card {
        0 => vec![],
        1 | 2 => vec![0],
        3 => vec![1, 0],
        4 => vec![1, 2, 0],
        5 => vec![2, 0],
        6 => vec![3, 1, 0],
        7 => vec![3, 4, 1, 2, 0],
        8 => vec![4, 5, 1, 2, 0],
        9 => vec![5, 2, 0],
        10 => vec![6, 3, 1, 0],
        11 => vec![6, 7, 3, 4, 1, 2, 0],
        12 => vec![7, 8, 3, 4, 5, 1, 2, 0],
        13 => vec![8, 9, 4, 5, 1, 2, 0],
        14 => vec![9, 5, 2, 0],
        15 => vec![10, 6, 3, 1, 0],
        16 => vec![10, 11, 6, 7, 3, 4, 1, 2, 0],
        17 => vec![11, 12, 6, 7, 8, 3, 4, 5, 1, 2, 0],
        18 => vec![12, 13, 7, 8, 9, 3, 4, 5, 1, 2, 0],
        19 => vec![13, 14, 8, 9, 4, 5, 1, 2, 0],
        20 => vec![14, 9, 5, 2, 0],
        21 => vec![15, 10, 6, 3, 1, 0],
        22 => vec![15, 16, 10, 11, 6, 7, 3, 4, 1, 2, 0],
        23 => vec![16, 17, 10, 11, 12, 6, 7, 8, 3, 4, 5, 1, 2, 0],
        24 => vec![17, 18, 11, 12, 13, 6, 7, 8, 9, 3, 4, 5, 1, 2, 0],
        25 => vec![18, 19, 12, 13, 14, 7, 8, 9, 4, 5, 1, 2, 0],
        26 => vec![19, 14, 9, 5, 2, 0],
        27 => vec![20, 15, 10, 6, 3, 1, 0],
        _ => vec![],
    }
}

pub fn card_blocked_by(card: usize) -> Vec<usize> {
    match card {
        0 => (1..28).collect(),
        1 => vec![
            3, 4, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26,
        ],
        2 => vec![
            4, 5, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20, 22, 23, 24, 25, 26, 27,
        ],
        3 => vec![6, 7, 10, 11, 12, 15, 16, 17, 18, 21, 22, 23, 24, 25],
        4 => vec![7, 8, 11, 12, 13, 16, 17, 18, 19, 22, 23, 24, 25, 26],
        5 => vec![8, 9, 12, 13, 14, 17, 18, 19, 20, 23, 24, 25, 26, 27],
        6 => vec![10, 11, 15, 16, 17, 21, 22, 23, 24],
        7 => vec![11, 12, 16, 17, 18, 22, 23, 24, 25],
        8 => vec![12, 13, 17, 18, 19, 23, 24, 25, 26],
        9 => vec![13, 14, 18, 19, 20, 24, 25, 26, 27],
        10 => vec![15, 16, 21, 22, 23],
        11 => vec![16, 17, 22, 23, 24],
        12 => vec![17, 18, 23, 24, 25],
        13 => vec![18, 19, 24, 25, 26],
        14 => vec![19, 20, 25, 26, 27],
        15 => vec![21, 22],
        16 => vec![22, 23],
        17 => vec![23, 24],
        18 => vec![24, 25],
        19 => vec![25, 26],
        20 => vec![26, 27],
        _ => vec![],
    }
}
