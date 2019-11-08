// Original Author: https://exercism.io/tracks/rust/exercises/book-store/solutions/e47a877f9a26467a9e844d18fd02a538
pub fn lowest_price(books: &Vec<u32>) -> u32 {
    if books.is_empty() {
        0
    } else {
        let mut current_prize = 0;
        let mut basket = vec![];
        let mut remaining_books = books.to_vec();
        let mut i = 0;
        while i < remaining_books.len() {
            if !basket.contains(&remaining_books[i]) {
                basket.push(remaining_books[i]);
                remaining_books.remove(i);
                let partial_prize = lowest_price(&remaining_books);
                let prize = partial_prize + match basket.len() {
                    1 => 8 * 100,
                    2 => 16 * 95,
                    3 => 24 * 90,
                    4 => 32 * 80,
                    5 => 40 * 75,
                    _ => unreachable!()
                };
                if current_prize == 0 || current_prize > prize { current_prize = prize };
            } else {
                i += 1;
            }
        }
        current_prize
    }
}