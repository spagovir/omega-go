use crate::board;
use std::iter::Flatten;
use std::iter::Map;

pub struct KoTrie {
    depth : u16,
    key : usize,
    children : Vec<Option<KoTrie>>,
}
const FIVE_MASK:u64 = 31;
struct FiveRowAnnotate(board::Row,u8);
impl Iterator for FiveRowAnnotate {
    type Item = usize;
    fn next(&mut self) -> Option <Self::Item> { // Consumes a row by iteratively taking the last 2 bits and shifting right.
        let FiveRowAnnotate(board::Row(row),size) = *self;
        if size > 250 {
            None
        } else {
            *self = FiveRowAnnotate(board::Row(row>>5), size-5);
            Some((row & FIVE_MASK) as usize)
        }
    }
}
// fn iterateBoard(board:&board::Board) -> Flatten<Map<std::vec::IntoIter<board::Row>, fn(board::Row) -> FiveRowAnnotate>> {
//     board.tiles.into_iter().map(|row| FiveRowAnnotate(row,(board.rows * 2))).flatten()
// }

fn lookUpAndAddR(koTrie:Option<KoTrie>, boardIter:&impl Iterator<Item=usize>) -> (Option<KoTrie>, bool) {
    (None,false)
}
pub fn lookUpAndAdd(koTrie:Option<KoTrie>, board:&board::Board) -> (Option<KoTrie>, bool) {
    lookUpAndAddR(koTrie, &(&board.tiles).into_iter().map(|row| FiveRowAnnotate(*row,(board.rows * 2))).flatten())
}
