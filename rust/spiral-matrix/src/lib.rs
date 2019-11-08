enum State {
    MovingRight,
    MovieDown,
    MovingLeft,
    MovingUp,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    if size == 0 {
        return Vec::new();
    } else if size == 1 {
        return vec![vec![1; size]; size];
    }
    let mut spiral: Vec<Vec<u32>> = vec![vec![0; size]; size];

    let mut i = 0;
    let mut j = 0;
    let mut val = 1;
    let mut moving = State::MovingRight;

    while spiral[i][j] == 0 {
        spiral[i][j] = val;
        val += 1;
        match moving {
            State::MovingRight => {
                if j == size - 1 || spiral[i][j + 1] != 0 {
                    i += 1;
                    moving = State::MovieDown;
                } else {
                    j += 1;
                }
            }
            State::MovieDown => {
                if i == size - 1 || spiral[i + 1][j] != 0 {
                    j -= 1;
                    moving = State::MovingLeft;
                } else {
                    i += 1;
                }
            }
            State::MovingLeft => {
                println!("{:?}", (i, j));
                if j == 0 || spiral[i][j - 1] != 0 {
                    i -= 1;
                    moving = State::MovingUp
                } else {
                    j -= 1;
                }
            }
            State::MovingUp => {
                if i == 0 || spiral[i - 1][j] != 0 {
                    j += 1;
                    moving = State::MovingRight;
                } else {
                    i -= 1
                }
            }
        }
    }

    spiral
}
