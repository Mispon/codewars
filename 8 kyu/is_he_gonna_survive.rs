/// Author: users/MMortaga
/// 
/// Kata:
/// A hero is on his way to the castle to complete his mission.
/// However, he's been told that the castle is surrounded with a couple of powerful dragons!
/// Each dragon takes 2 bullets to be defeated, our hero has no idea how many bullets he should carry.
/// Assuming he's gonna grab a specific given number of bullets and move forward
/// to fight another specific given number of dragons, will he survive?
/// 
/// Return True if yes, False otherwise :)

fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= dragons * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hero(10, 5), true);
        assert_eq!(hero(7, 4), false);
        assert_eq!(hero(4, 5), false);
        assert_eq!(hero(100, 40), true);
        assert_eq!(hero(1500, 751), false);
        assert_eq!(hero(0, 1), false);
    }
}