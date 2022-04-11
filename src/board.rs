// A go board is an array of size (8*ceiling(width/16) * width) bytes coupled with a ko trie. 
// A row is 32 2-bit tiles packed into a single 8 byte word. 

#[derive(FromPrimitive, ToPrimitive)]
enum Tile {
    Empty == 0;
    Black == 1;
    White == 2; 
}
fn display_tile(tile:Tile) -> char {
    match tile {
        Empty => '.',
        Black => 'x',
        White => 'o',
    }
}
enum Player {
    BlackP;
    WhiteP;
}
fn display_player(player:Player) -> &str {
    match player {
        BlackP => "BLACK"; 
        WhiteP => "WHITE";
    }
}
TILE_MASK:u64 = 3;
struct Row(u64);
impl Iterator for (Row,u8) {
    type Item = Tile;
    fn next(&mut self) -> Option <Self::Item> {
        let (row,size) = *self in
            if size == 0 {
                None 
            } else {
                *self = (row>>2, size--);
                FromPrimitive::from_i64(row & TILE_MASK)
            }
    }
}

struct Board {
    rows :: u8;
    tiles :: Vec<Row>; 
    ko :: Option<KoTrie>;
    player :: Player;
    pass :: bool;
}
struct KoTrie {
    depth :: u16;
    key :: u8;
    children :: Vec<Option<KoTrie>>;
}
fn show_board(board:&Board) -> String  {
    let mut s = String::with_capacity((board.rows + 1) * board.rows +  20);
    for row in board.tiles  {
        for tile in (row, board.rows) {
            s.push(display_tile(tile));
        }
        s.push('\n');
    }
    s.push_str(display_player(board.player));
    s.push_str(" to move.\n");
}
fn init_board(rows:u8) -> {
    Board {
        rows,
        tiles: vec![0;rows],
        ko : None, // temporary, change later;
        player : Black,
        pass : false,
    }
}





