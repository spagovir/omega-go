use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;


// A go board is an array of size (8*ceiling(width/16) * width) bytes coupled with a ko trie. 
// A row is 32 2-bit tiles packed into a single 8 byte word. 
#[derive(num_derive::FromPrimitive, num_derive::ToPrimitive,Copy,Clone)]
pub enum Tile {
    Empty = 0,
    Black = 1,
    White = 2, 
}
pub fn display_tile(tile:Tile) -> char {
    match tile {
        Tile::Empty => '.',
        Tile::Black => 'x',
        Tile::White => 'o',
    }
}
#[derive(Copy,Clone)]
pub enum Player {
    BlackP,
    WhiteP,
}
pub fn display_player(player:Player) -> &'static str {
    match player {
        Player::BlackP => "BLACK",
        Player::WhiteP => "WHITE",
    }
}

const TILE_MASK:u64 = 3; // Selects the last 2 bits of a u64 (a go tile has 3 states, empty, black, or white, 
// and is thus encoded in 2 bits. 
#[derive(Copy,Clone)]
pub struct Row(u64);
#[derive(Copy,Clone)]
pub struct RowAnnotate(Row,u8);
impl Iterator for RowAnnotate {
    type Item = Tile;
    fn next(&mut self) -> Option <Self::Item> { // Consumes a row by iteratively taking the last 2 bits and shifting right. 
        let RowAnnotate(Row(row),size) = *self;
            if size == 0 {
                None 
            } else {
                *self = RowAnnotate(Row(row>>2), size-1);
                num_traits::FromPrimitive::from_u64(row & TILE_MASK)
            }
    }
}

pub struct Board {
    rows : u8,
    tiles : Vec<Row>,
    ko : Option<KoTrie>,
    player : Player,
    pass : bool,
}
pub struct KoTrie {
    depth : u16,
    key : u8,
    children : Vec<Option<KoTrie>>,
}
pub fn show_board(board:&Board) -> String  {
    let mut s = String::with_capacity(2 * (usize::from(board.rows) + 1) * usize::from(board.rows) +  20);
    for row in &board.tiles  {
        for tile in RowAnnotate(*row, board.rows) {
            s.push(display_tile(tile));
            s.push(' ');
        }
        s.push('\n');
    }
    s.push_str(display_player(board.player));
    s.push_str(" to move.\n");
    s
}
pub fn init_board(rows:u8) -> Board {
    Board {
        rows,
        tiles: vec![Row(0);rows.into()],
        ko : None, // temporary, change later;
        player : Player::BlackP,
        pass : false,
    }
}





