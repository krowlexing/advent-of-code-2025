use std::fs;

pub struct Rotation {
    direction: Direction,
    count: i16,
}

pub enum Direction {
    Left,
    Right,
}

fn read_rotations() -> Vec<Rotation> {
    let rotations_string = fs::read_to_string("data/rotations.txt").unwrap();

    rotations_string
        .lines()
        .map(parse_rotation)
        .collect::<Vec<_>>()
}

fn parse_rotation(line: &str) -> Rotation {
    let letters = &line.chars().collect::<Vec<_>>();
    let letter = letters[0];
    let number = &line[1..];

    let number: i16 = number.parse().unwrap();

    let direction = match letter {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => {
            panic!("wrong direction")
        }
    };

    Rotation {
        direction,
        count: number,
    }
}

fn main() {
    solve_password();
    let LockResult {
        passed_zero,
        new_position,
    } = solve_password_v2();

    println!("--\n{passed_zero} - {new_position}")
}

fn solve_password() {
    use Direction::*;

    let rotations = read_rotations();

    let mut position = 50;
    let mut zero_count = 0;

    for rotation in rotations {
        position = match rotation.direction {
            Left => circle_sub(position, rotation.count),
            Right => circle_add(position, rotation.count),
        };

        if position == 0 {
            zero_count += 1;
        }
    }

    println!("{zero_count} - {position}")
}

fn circle_add(pos: i16, count: i16) -> i16 {
    (pos + count) % 100
}

fn circle_sub(pos: i16, count: i16) -> i16 {
    let count = count % 100;

    if (pos - count) < 0 {
        (pos + (100 - count)) % 100
    } else {
        pos - count
    }
}

fn solve_password_v2() -> LockResult {
    use Direction::*;
    let rotations = read_rotations();

    let mut position = 50;

    let mut zero_count = 0;

    for Rotation { direction, count } in rotations {
        let lock_result = match direction {
            Left => circle_sub_v2(position, count),
            Right => circle_add_v2(position, count),
        };

        let LockResult {
            passed_zero,
            new_position,
        } = lock_result;

        position = new_position;

        zero_count += passed_zero;

        println!("{position} - {zero_count}");
    }

    LockResult {
        passed_zero: zero_count,
        new_position: position,
    }
}

pub struct LockResult {
    passed_zero: i16,
    new_position: i16,
}

fn circle_add_v2(pos: i16, count: i16) -> LockResult {
    let full_rotations = count / 100;
    let count = count % 100;

    let new_position = pos + count;

    let passed_zero = new_position >= 100;
    let passed_zero = if passed_zero { 1 } else { 0 };

    let new_position = new_position % 100;

    if new_position == 0 {
        LockResult {
            passed_zero: full_rotations + 1,
            new_position,
        }
    } else {
        LockResult {
            passed_zero: full_rotations + passed_zero,
            new_position,
        }
    }
}

fn circle_sub_v2(pos: i16, count: i16) -> LockResult {
    let full_rotations = count / 100;
    let count = count % 100;

    let new_position = pos - count;

    let passed_zero = new_position < 0;

    let new_position = if (pos - count) < 0 {
        (pos + (100 - count)) % 100
    } else {
        pos - count
    };

    let passed_zero = if passed_zero && pos != 0 { 1 } else { 0 };

    if new_position == 0 {
        LockResult {
            passed_zero: full_rotations + 1,
            new_position,
        }
    } else {
        LockResult {
            passed_zero: full_rotations + passed_zero,
            new_position,
        }
    }
}

#[test]
fn test_circle_add() {
    assert_eq!(circle_add(0, 100), 0);
    assert_eq!(circle_add(1, 99), 0);
    assert_eq!(circle_add(1, 100), 1);
    assert_eq!(circle_add(50, 100), 50);
}

#[test]
fn test_circle_sub() {
    assert_eq!(circle_sub(0, 100), 0);
    assert_eq!(circle_sub(1, 99), 2);
    assert_eq!(circle_sub(1, 100), 1);
    assert_eq!(circle_sub(50, 149), 1);
}
