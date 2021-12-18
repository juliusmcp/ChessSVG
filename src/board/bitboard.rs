
 use crate::board::piecetype::PieceType;
 use crate::board::piecetype::Colour;
 use crate::board::square::Square;
 
 const INIT_WHITE_PAWNS: u64 = 0xFF_00;
 const INIT_BLACK_PAWNS: u64 = 0xFF_00_00_00_00_00_00;
 const INIT_WHITE_KING: u64 = 0x08;
 const INIT_BLACK_KING: u64 = 0x08_00_00_00_00_00_00_00;
 const INIT_WHITE_QUEEN: u64 = 0x10;
 const INIT_BLACK_QUEEN: u64 = 0x1000000000000000;
 const INIT_WHITE_BISHOPS: u64 = 0x24;
 const INIT_BLACK_BISHOPS: u64 =  0x2400000000000000;
 const INIT_WHITE_KNIGHTS: u64 = 0x42;
 const INIT_BLACK_KNIGHTS: u64 = 0x4200000000000000;
 const INIT_WHITE_ROOKS: u64 = 0x81;
 const INIT_BLACK_ROOKS: u64 = 0x8100000000000000;

 pub struct BitBoard(pub u64, pub PieceType);

 impl BitBoard {
     pub fn new(configuration: PieceType) -> BitBoard  {
         match configuration {
             PieceType::Pawn(Colour::White) => BitBoard(INIT_WHITE_PAWNS, configuration),
             PieceType::Pawn(Colour::Black) => BitBoard(INIT_BLACK_PAWNS, configuration),
             PieceType::King(Colour::White) => BitBoard(INIT_WHITE_KING, configuration),
             PieceType::King(Colour::Black) => BitBoard(INIT_BLACK_KING, configuration),
             PieceType::Queen(Colour::White) => BitBoard(INIT_WHITE_QUEEN, configuration),
             PieceType::Queen(Colour::Black) => BitBoard(INIT_BLACK_QUEEN, configuration),
             PieceType::Bishop(Colour::White) => BitBoard(INIT_WHITE_BISHOPS, configuration),
             PieceType::Bishop(Colour::Black) => BitBoard(INIT_BLACK_BISHOPS, configuration),
             PieceType::Knight(Colour::White) => BitBoard(INIT_WHITE_KNIGHTS, configuration),
             PieceType::Knight(Colour::Black) => BitBoard(INIT_BLACK_KNIGHTS, configuration),
             PieceType::Rook(Colour::White) => BitBoard(INIT_WHITE_ROOKS, configuration),
             PieceType::Rook(Colour::Black) => BitBoard(INIT_BLACK_ROOKS, configuration),
         }
     }
     
     pub fn set_square(&mut self, square: &Square) {
         self.0 |= square.value();
     }
     pub fn unset_square(&mut self, square: &Square) {
         self.0 &= square.value();
     }   
     pub fn get_square(&self, square: &Square) -> bool {
        return self.0 & square.value() !=0
     }
     pub fn print_bytes(&self) {
         for byte in self.0.to_be_bytes().iter() {
             println!("{:08b}", byte);
         }
         println!("{}", "");
         
     }
 }   



