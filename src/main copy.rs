/*#[allow(dead_code)]

pub enum Colour {
    White,
    Black,
}

pub enum PieceType {
    Pawn(Colour),
    King(Colour),
    Queen(Colour),
    Bishop(Colour),
    Knight(Colour),
    Rook(Colour),
 
}

pub enum Square
{
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
    
}

impl Square {
    /*pub fn new(square: &str) -> Self {
        let sq: Vec<_> = square.chars().collect();
        if sq.len()>2 {
            Square::A1;
        }
        if sq[1] as u8 >8 {
            Square::A1;
        }
        match &sq[0] {
            'a1' => Square::A1,
            'b1' => Square::B1,
            'c1' => Square::C1,
            'd' => Square::D(sq[1] as u8),
            'e' => Square::E(sq[1] as u8),
            'f' => Square::F(sq[1] as u8),
            'g' => Square::G(sq[1] as u8),
            'h' => Square::H(sq[1] as u8),
            'A' => Square::A(sq[1] as u8),
            'B' => Square::B(sq[1] as u8),
            'C' => Square::C(sq[1] as u8),
            'D' => Square::D(sq[1] as u8),
            'E' => Square::E(sq[1] as u8),
            'F' => Square::F(sq[1] as u8),
            'G' => Square::G(sq[1] as u8),
            'H' => Square::H(sq[1] as u8),
            _=>Square::A(sq[1] as u8),
        }
    }*/

    pub fn value(&self) -> u64 {
        match *self {
            Square::A1 => 0x80,
            Square::B1 => 0x40,
            Square::C1 => 0x20,
            Square::D1 => 0x10,
            Square::E1 => 0x8,
            Square::F1 => 0x4,
            Square::G1 => 0x2,
            Square::H1 => 0x1,
            Square::A2 => 0x8000,
            Square::B2 => 0x4000,
            Square::C2 => 0x2000,
            Square::D2 => 0x1000,
            Square::E2 => 0x800,
            Square::F2 => 0x400,
            Square::G2 => 0x200,
            Square::H2 => 0x100,
            Square::A3 => 0x800000,
            Square::B3 => 0x400000,
            Square::C3 => 0x200000,
            Square::D3 => 0x100000,
            Square::E3 => 0x80000,
            Square::F3 => 0x40000,
            Square::G3 => 0x20000,
            Square::H3 => 0x10000,
            Square::A4 => 0x80000000,
            Square::B4 => 0x40000000,
            Square::C4 => 0x20000000,
            Square::D4 => 0x10000000,
            Square::E4 => 0x8000000,
            Square::F4 => 0x4000000,
            Square::G4 => 0x2000000,
            Square::H4 => 0x1000000,
            Square::A5 => 0x8000000000,
            Square::B5 => 0x4000000000,
            Square::C5 => 0x2000000000,
            Square::D5 => 0x1000000000,
            Square::E5 => 0x800000000,
            Square::F5 => 0x400000000,
            Square::G5 => 0x200000000,
            Square::H5 => 0x100000000,
            Square::A6 => 0x800000000000,
            Square::B6 => 0x400000000000,
            Square::C6 => 0x200000000000,
            Square::D6 => 0x100000000000,
            Square::E6 => 0x80000000000,
            Square::F6 => 0x40000000000,
            Square::G6 => 0x20000000000,
            Square::H6 => 0x10000000000,
            Square::A7 => 0x80000000000000,
            Square::B7 => 0x40000000000000,
            Square::C7 => 0x20000000000000,
            Square::D7 => 0x10000000000000,
            Square::E7 => 0x8000000000000,
            Square::F7 => 0x4000000000000,
            Square::G7 => 0x2000000000000,
            Square::H7 => 0x1000000000000,
            Square::A8 => 0x8000000000000000,
            Square::B8 => 0x4000000000000000,
            Square::C8 => 0x2000000000000000,
            Square::D8 => 0x1000000000000000,
            Square::E8 => 0x800000000000000,
            Square::F8 => 0x400000000000000,
            Square::G8 => 0x200000000000000,
            Square::H8 => 0x100000000000000,
        }
    }

}
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

pub struct BitBoard(pub u64, pub PieceType);

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
fn main() {
    let mut board = Board::new();
    board.clear();
    //board.print(PieceType::King(Colour::Black));
    let fen= "4r1k1/p4p1p/8/3p4/8/6NP/P1P4P/4q1bK w - - 2 28".to_string();
    board.from_fen(&fen);
  
    //board.set_square(Square::A5,PieceType::Pawn(Colour::White));
    //board.set_square(Square::A6,PieceType::Pawn(Colour::White));
    //board.set_square(Square::A7,PieceType::Pawn(Colour::White));
    //board.set_square(Square::A8,PieceType::Pawn(Colour::White));
    board.print(PieceType::King(Colour::Black));
   // println!("active: {:?}", board.colour_move)
}

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
    
    fn set_square(&mut self, square: Square) {
        self.0 |= square.value();
    }
    fn unset_square(&mut self, square: Square) {
        self.0 &= square.value();
    }   
    fn print_bytes(&self) {
        for byte in self.0.to_be_bytes().iter() {
            println!("{:08b}", byte);
        }
        println!("{}", "");
    }
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
    pub fn print(&self, configuration: PieceType) {
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

    
    pub fn identify_square(&self, file: u8, rank: u8) -> Square {
        match (file, rank)  {
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
            (5, 8) => Square::E8,
            (6, 1) => Square::F1,
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
    pub fn set_square(&mut self, square: Square, piece: PieceType) {
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

    
    pub fn from_fen(&mut self, fen: &str) {
        let parts: Vec<_> = fen.split_whitespace().collect();
        if parts.len() != 6 {
           // return Err(FenError::NotEnoughParts);
        }
        self.clear();
        let ranks=parts[0];
        let mut rank: u8=8;
        let mut file: u8=0;

        for ch in ranks.chars() {
            match ch {
                'p'  => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Pawn(Colour::Black))
                 },
                'r'  => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Rook(Colour::Black))
                 },
                'n'  => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Knight(Colour::Black))
                 },
                'b'  => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Bishop(Colour::Black))
                 },
                'q'  => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Queen(Colour::Black))
                 },
                'k'  =>{ 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::King(Colour::Black))
                 },
                'P' => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Pawn(Colour::White))
                 },
                'R' => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Rook(Colour::White))
                 },
                'N' => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Knight(Colour::White))
                 },
                'B' => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Bishop(Colour::White))
                 },
                'Q' => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::Queen(Colour::White))
                 },
                'K' => { 
                    file= file +1;
                    self.set_square_co_ords(file, rank, PieceType::King(Colour::White))
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

        /*for ch in ranks.chars() {
            match ch.to_digit(10) {
                Some(spaces)=> {
                    println!("spaces {:?}", spaces);
                   // file_id =file_id + spaces as u8;
                    let new_file= spaces as u8+ file_id;
                   // println!("file");
                  //  println!("{:?}",file_id);
                    for _n in file_id..new_file {
                       file_id = file_id +1;
                       println!("file {:?}",file_id);
                    }

                }
                None=> {
           
                    match ch {
                        '/' => {
                            rank_id=rank_id-1;
                            file_id=0
                        },
                        _ => file_id =file_id + 1,
                    }
                    println!("ch {} file {} rank {}",ch, file_id,rank_id);

                    let current_square=self.identify_square(file_id, rank_id);
                    self.white_pawns.set_square(current_square);
                }
            }
            //rank_id=rank_id+1;
        }*/

    }
    

   
    fn set_square_co_ords(&mut self, file :u8, rank :u8, piece: PieceType) {
        let current_square=self.identify_square(file, rank);
        self.set_square(current_square, piece);
    }
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
}