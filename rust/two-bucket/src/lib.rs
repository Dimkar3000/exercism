use std::collections::VecDeque;
#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

impl Default for Bucket {
    fn default() -> Self {
        Bucket::One
    }
}
#[derive(PartialEq, Eq, Debug, Default)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

fn transfer(volume: u8, bucket: u8, capacity: u8) -> (u8, u8) {
    if volume + bucket > capacity {
        (volume + bucket - capacity, capacity)
    } else {
        (0, bucket + volume)
    }
}
use std::collections::HashSet;

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let mut queue: VecDeque<(u8, u8, u8)> = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((0, 0, 0));

    // This state should never happen so I added to the seen
    if start_bucket == &Bucket::One {
        seen.insert((0, capacity_2));
    } else {
        seen.insert((capacity_1, 0));
    }

    while !queue.is_empty() {
        if let Some((steps, one, two)) = queue.pop_front() {
            // Break the state loops
            if seen.contains(&(one, two)) {
                continue;
            }
            seen.insert((one, two));

            // Check if the result was found
            if one == goal {
                return BucketStats {
                    moves: steps,
                    goal_bucket: Bucket::One,
                    other_bucket: two,
                };
            } else if two == goal {
                return BucketStats {
                    moves: steps,
                    goal_bucket: Bucket::Two,
                    other_bucket: one,
                };
            }

            // push all possible states to the queue

            // Fill
            queue.push_back((steps + 1, one, capacity_2));
            queue.push_back((steps + 1, capacity_1, two));

            // Empty
            queue.push_back((steps + 1, one, 0));
            queue.push_back((steps + 1, 0, two));

            // Transfer
            let (temp_b1, temp_b2) = transfer(one, two, capacity_2);
            queue.push_back((steps + 1, temp_b1, temp_b2));

            let (temp_b2, temp_b1) = transfer(two, one, capacity_1);
            queue.push_back((steps + 1, temp_b1, temp_b2));
        } else {
            break;
        }
    }

    BucketStats::default()
}
