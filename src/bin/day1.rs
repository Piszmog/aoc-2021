use aoc_2021::shared;

fn main() {
    println!("{}", shared::hello_world());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eq() {
        assert_eq!(1, 1);
    }
}