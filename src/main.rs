#[allow(dead_code)]

mod board;

use crate::board::board::Board;
use crate::board::piecetype::PieceType;
use crate::board::piecetype::Colour;

fn main() {
    let mut board = Board::new();
    //board.clear();
    //let fen= "4r1k1/p4p1p/8/3p4/8/6NP/P1P4P/4q1bK w - - 2 28".to_string();
   // board.from_fen(&fen);
    board.print();
    board.generate_svg("board.svg", 45);
   // println!("active: {:?}", board.colour_move)
}
