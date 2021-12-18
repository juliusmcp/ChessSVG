use crate::board::bitboard::BitBoard;
use crate::board::piecetype::PieceType;
use crate::board::piecetype::Colour;
use crate::board::square::Square;
use crate::board::image;
  


    //copied
    pub enum FenError<'a> {
        NotEnoughParts,
        BadPlacement(&'a str),
        TooManyPieces(&'a str),
        UnknownPiece(char),
        NoSuchSide(&'a str),
        BadEnPassant(&'a str),
        BadHalfmove(&'a str),
        BadFullmove(&'a str),
    }


   

    pub struct Board {
        white_pawns: BitBoard,
        black_pawns: BitBoard,
        white_king: BitBoard,
        black_king: BitBoard,
        white_queen: BitBoard,
        black_queen: BitBoard,
        white_bishops: BitBoard,
        black_bishops: BitBoard,
        white_knights: BitBoard,
        black_knights: BitBoard,
        white_rooks: BitBoard,
        black_rooks: BitBoard,
        colour_move: Colour,
    
    }

    
    impl Board {
        pub fn new() -> Self {
            Board {
                white_pawns: BitBoard::new(PieceType::Pawn(Colour::White)),
                black_pawns: BitBoard::new(PieceType::Pawn(Colour::Black)),
                white_king: BitBoard::new(PieceType::King(Colour::White)),
                black_king: BitBoard::new(PieceType::King(Colour::Black)),
                white_queen: BitBoard::new(PieceType::Queen(Colour::White)),
                black_queen: BitBoard::new(PieceType::Queen(Colour::Black)),
                white_bishops: BitBoard::new(PieceType::Bishop(Colour::White)),
                black_bishops: BitBoard::new(PieceType::Bishop(Colour::Black)),
                white_knights: BitBoard::new(PieceType::Knight(Colour::White)),
                black_knights: BitBoard::new(PieceType::Knight(Colour::Black)),
                white_rooks: BitBoard::new(PieceType::Rook(Colour::White)),
                black_rooks: BitBoard::new(PieceType::Rook(Colour::Black)),
                colour_move: Colour::White,
            }
            
        }
        pub fn print_bitboard(&self, configuration: PieceType) {
            match configuration {
                PieceType::Pawn(Colour::White) => self.white_pawns.print_bytes(),
                PieceType::Pawn(Colour::Black) => self.black_pawns.print_bytes(),
                PieceType::King(Colour::White) => self.white_king.print_bytes(),
                PieceType::King(Colour::Black) => self.black_king.print_bytes(),
                PieceType::Queen(Colour::White) => self.white_queen.print_bytes(),
                PieceType::Queen(Colour::Black) => self.black_queen.print_bytes(),
                PieceType::Bishop(Colour::White) => self.white_bishops.print_bytes(),
                PieceType::Bishop(Colour::Black) => self.black_bishops.print_bytes(),
                PieceType::Knight(Colour::White) => self.white_knights.print_bytes(),
                PieceType::Knight(Colour::Black) => self.black_knights.print_bytes(),
                PieceType::Rook(Colour::White) => self.white_rooks.print_bytes(),
                PieceType::Rook(Colour::Black) => self.black_rooks.print_bytes(),
            
            }
        }

        pub fn print(&self) {
            for file in 8..0 {
                println!("----------------------------------------");
                for rank in 0..8 {
                    let square = self.identify_square(file, rank);
                    print!("| {}  ",self.get_unicode_char(&square));
                }
                print!("|\n");
            }
        }

        pub fn get_unicode_char(&self, square: &Square) -> String {
            let piece = self.get_square_piece(&square);
            match piece {
                Some(PieceType::Pawn(Colour::White)) => String::from("♟"),
                Some(PieceType::Pawn(Colour::Black)) => String::from("♙"),
                Some(PieceType::King(Colour::White)) => String::from("♔"),
                Some(PieceType::King(Colour::Black)) => String::from("♚"),
                Some(PieceType::Queen(Colour::White)) => String::from("♕"),
                Some(PieceType::Queen(Colour::Black)) => String::from("♛"),
                Some(PieceType::Bishop(Colour::White)) => String::from("♗"),
                Some(PieceType::Bishop(Colour::Black))=> String::from("♝"),
                Some(PieceType::Knight(Colour::White))=> String::from("♘"),
                Some(PieceType::Knight(Colour::Black))=> String::from("♞"),
                Some(PieceType::Rook(Colour::White)) => String::from("♖"),
                Some(PieceType::Rook(Colour::Black)) => String::from("♜"),
                None => String::from(" "),
            }
        }
        
        pub fn identify_square(&self, file: u8, rank: u8) -> Square {
            match (rank, file)  {
                (1, 1) => Square::A1,
                (1, 2) => Square::A2,
                (1, 3) => Square::A3,
                (1, 4) => Square::A4,
                (1, 5) => Square::A5,
                (1, 6) => Square::A6,
                (1, 7) => Square::A7,
                (1, 8) => Square::A8,
                (2, 1) => Square::B1,
                (2, 2) => Square::B2,
                (2, 3) => Square::B3,
                (2, 4) => Square::B4,
                (2, 5) => Square::B5,
                (2, 6) => Square::B6,
                (2, 7) => Square::B7,
                (2, 8) => Square::B8,
                (3, 1) => Square::C1,
                (3, 2) => Square::C2,
                (3, 3) => Square::C3,
                (3, 4) => Square::C4,
                (3, 5) => Square::C5,
                (3, 6) => Square::C6,
                (3, 7) => Square::C7,
                (3, 8) => Square::C8,
                (4, 1) => Square::D1,
                (4, 2) => Square::D2,
                (4, 3) => Square::D3,
                (4, 4) => Square::D4,
                (4, 5) => Square::D5,
                (4, 6) => Square::D6,
                (4, 7) => Square::D7,
                (4, 8) => Square::D8,
                (5, 1) => Square::E1,
                (5, 2) => Square::E2,
                (5, 3) => Square::E3,
                (5, 4) => Square::E4,
                (5, 5) => Square::E5,
                (5, 6) => Square::E6,
                (5, 7) => Square::E7,
                (5, 8) => Square::F1,
                (6, 2) => Square::F2,
                (6, 3) => Square::F3,
                (6, 4) => Square::F4,
                (6, 5) => Square::F5,
                (6, 6) => Square::F6,
                (6, 7) => Square::F7,
                (6, 8) => Square::F8,
                (7, 1) => Square::G1,
                (7, 2) => Square::G2,
                (7, 3) => Square::G3,
                (7, 4) => Square::G4,
                (7, 5) => Square::G5,
                (7, 6) => Square::G6,
                (7, 7) => Square::G7,
                (7, 8) => Square::G8,
                (8, 1) => Square::H1,
                (8, 2) => Square::H2,
                (8, 3) => Square::H3,
                (8, 4) => Square::H4,
                (8, 5) => Square::H5,
                (8, 6) => Square::H6,
                (8, 7) => Square::H7,
                (8, 8) => Square::H8,
                _ => Square::A1,
            }
        }
        pub fn set_square(&mut self, square: &Square, piece: &PieceType) {
            match piece {
                PieceType::Pawn(Colour::White) => self.white_pawns.set_square(square),
                PieceType::Pawn(Colour::Black) => self.black_pawns.set_square(square),
                PieceType::King(Colour::White) => self.white_king.set_square(square),
                PieceType::King(Colour::Black) => self.black_king.set_square(square),
                PieceType::Queen(Colour::White) => self.white_queen.set_square(square),
                PieceType::Queen(Colour::Black) => self.black_queen.set_square(square),
                PieceType::Bishop(Colour::White) => self.white_bishops.set_square(square),
                PieceType::Bishop(Colour::Black) => self.black_bishops.set_square(square),
                PieceType::Knight(Colour::White) => self.white_knights.set_square(square),
                PieceType::Knight(Colour::Black) => self.black_knights.set_square(square),
                PieceType::Rook(Colour::White) => self.white_rooks.set_square(square),
                PieceType::Rook(Colour::Black) => self.black_rooks.set_square(square),
            }
        }

        pub fn set_square_co_ords(&mut self, file :u8, rank :u8, piece: &PieceType) {
            let current_square= self.identify_square(file, rank);
            self.set_square(&current_square, piece);
        }

        pub fn get_square_piece(&self, square: &Square) -> Option<&PieceType>{
            if self.white_pawns.get_square(square) {
                return Some(&PieceType::Pawn(Colour::White))
            }
            if self.black_pawns.get_square(square) {
                return Some(&PieceType::Pawn(Colour::Black))
            }
            if self.white_rooks.get_square(square) {
                return Some(&PieceType::Rook(Colour::White))
            }
            if self.black_rooks.get_square(square) {
                return Some(&PieceType::Rook(Colour::Black))
            }
            if self.white_knights.get_square(square) {
                return Some(&PieceType::Knight(Colour::White))
            }
            if self.black_knights.get_square(square) {
                return Some(&PieceType::Knight(Colour::Black))
            }
            if self.white_bishops.get_square(square) {
                return Some(&PieceType::Bishop(Colour::White))
            }
            if self.black_bishops.get_square(square) {
                return Some(&PieceType::Bishop(Colour::Black))
            }
            if self.white_queen.get_square(square) {
                return Some(&PieceType::Queen(Colour::White))
            }
            if self.black_queen.get_square(square) {
                return Some(&PieceType::Queen(Colour::Black))
            }
            if self.white_king.get_square(square) {
                return Some(&PieceType::King(Colour::White))
            }
            if self.black_king.get_square(square) {
                return Some(&PieceType::King(Colour::Black))
            }
            None
        }
        pub fn from_fen(&mut self, fen: &str) {
            self.clear();
            let parts: Vec<_> = fen.split_whitespace().collect();
            if parts.len() != 6 {
            // return Err(FenError::NotEnoughParts);
            }
            let ranks=parts[0];
            let mut rank: u8=8;
            let mut file: u8=0;

            for ch in ranks.chars() {
                match ch {
                    'p'  => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Pawn(Colour::Black))
                    },
                    'r'  => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Rook(Colour::Black))
                    },
                    'n'  => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Knight(Colour::Black))
                    },
                    'b'  => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Bishop(Colour::Black))
                    },
                    'q'  => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Queen(Colour::Black))
                    },
                    'k'  =>{ 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::King(Colour::Black))
                    },
                    'P' => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Pawn(Colour::White))
                    },
                    'R' => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Rook(Colour::White))
                    },
                    'N' => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Knight(Colour::White))
                    },
                    'B' => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Bishop(Colour::White))
                    },
                    'Q' => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::Queen(Colour::White))
                    },
                    'K' => { 
                        file= file +1;
                        self.set_square_co_ords(file, rank, &PieceType::King(Colour::White))
                    },
                    '1'..='8' => {
                    match char::to_digit(ch, 10) {
                            Some(spaces)=> {
                                file= file + spaces as u8;
                            },
                            None => {

                            }
                    }
                    },
                    '/' => {
                        rank=rank-1;
                        file=0;
                    },
                    _ => println!("found unexpected token {:?} in file {:?} rank {:?}", ch, file,rank),
                };

                if parts[1]=="b" {
                    self.colour_move=Colour::Black;
                } else {
                    self.colour_move=Colour::White;
                }
            }

        }

       /* pub fn set_square_colour(&mut self, square: Square, r :u8, g :u8, b :u8) {
            
        }
        
        pub fn set_square_colour_co_ords(&mut self, file :u8, rank :u8, r :u8, g :u8, b :u8) {
            let current_square=self.identify_square(file, rank);
            self.set_square_colour(current_square, r, g, b);
        }*/

        pub fn clear(&mut self) {
            self.white_pawns.0=0;
            self.white_queen.0=0;
            self.white_king.0=0;
            self.white_bishops.0=0;
            self.white_knights.0=0;
            self.white_rooks.0=0;
            self.black_pawns.0=0;
            self.black_queen.0=0;
            self.black_king.0=0;
            self.black_bishops.0=0;
            self.black_knights.0=0;
            self.black_rooks.0=0;
        }

        pub fn generate_svg(&self, path: &str, square_width: i32) {
            image::create_svg(&self, path,square_width);
        }

      
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn identify_square_from_file_rank() {
            let mut board = Board::new();
            let a8= board.identify_square(1,8);
            assert_eq!(a8, Square::A8);
        }
    }