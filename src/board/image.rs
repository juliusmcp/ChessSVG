#[allow(dead_code)] 
#[allow(unused_variables)] 
use svg::Document;
use svg::node::Text;
use svg::node::element::Group;
use svg::node::element::Rectangle;
use crate::board::piecetype::PieceType;
use crate::board::piecetype::Colour;
use crate::board::square::Square;
use crate::board::board::Board;


pub fn create_svg(board : &Board, path :&str, square_width: i32)
{
    let mut boardlayer= draw_board(square_width);
    for file in 0..8 {
        for rank in 0..8 {
            let square = board.identify_square(file, rank);
            if let Some(piece) = board.get_square_piece(&square) {
                let svg_piece= draw_piece(&square, piece ,square_width);
                boardlayer = boardlayer.add(svg_piece);
            }
           
        }
    }
    let document = Document::new()
        .set("viewBox", (0, 0, 1400, 1900))
        .add(boardlayer);
    svg::save(path, &document).unwrap();
        

}

fn draw_board(square_width: i32) -> Group {
    let mut boardlayer= Group::new().set("id","board");
    for file in 0..8 {
        let x = file * square_width;
        for rank in 0..8 {
            let square_colour= get_square_colour(file, rank);
            let y = rank * square_width;
            let square = Rectangle::new()
            .set("width", square_width)
            .set("height", square_width)
            .set("x", x)
            .set("y", y)
            .set("style",square_colour);
             boardlayer = boardlayer.add(square);
        }
    }
    boardlayer
}


fn get_square_colour(file: i32, rank: i32) -> String {
    let calc= file + rank;
    let rgb = format!("fill:rgb({},{},{})", 165,117,81);
    if calc % 2 == 0 {
        //light
        return format!("fill:rgb({},{},{})", 235,209,166);
    }
    rgb
	
}

fn draw_piece(square: &Square, piece: &PieceType, square_width: i32) -> Group {
    let piece_position = get_piece_position(square, square_width);
    let pp = format!("translate({} {})", piece_position.0, piece_position.1);
    let piece_svg = choose_piece_svg(piece);
    let contents = Text::new(piece_svg);
    return Group::new()
        .set("id","test")
        .set("transform",pp)
        .add(contents);
}

fn get_piece_position(square: &Square, square_width: i32) -> (i32, i32) {
    match square {
        Square::A1 => (0,square_width*7),
        Square::B1 => (square_width*1,square_width*7),
        Square::C1 => (square_width*2,square_width*7),
        Square::D1 => (square_width*3,square_width*7),
        Square::E1 => (square_width*4,square_width*7),
        Square::F1 => (square_width*5,square_width*7),
        Square::G1 => (square_width*6,square_width*7),
        Square::H1 => (square_width*7,square_width*7),
        Square::A2 => (0,square_width*6),
        Square::B2 => (square_width*1,square_width*6),
        Square::C2 => (square_width*2,square_width*6),
        Square::D2 => (square_width*3,square_width*6),
        Square::E2 => (square_width*4,square_width*6),
        Square::F2 => (square_width*5,square_width*6),
        Square::G2 => (square_width*6,square_width*6),
        Square::H2 => (square_width*7,square_width*6),
        Square::A3 => (0,square_width*5),
        Square::B3 => (square_width*1,square_width*5),
        Square::C3 => (square_width*2,square_width*5),
        Square::D3 => (square_width*3,square_width*5),
        Square::E3 => (square_width*4,square_width*5),
        Square::F3 => (square_width*5,square_width*5),
        Square::G3 => (square_width*6,square_width*5),
        Square::H3 => (square_width*7,square_width*5),
        Square::A4 => (0,square_width*4),
        Square::B4 => (square_width*1,square_width*4),
        Square::C4 => (square_width*2,square_width*4),
        Square::D4 => (square_width*3,square_width*4),
        Square::E4 => (square_width*4,square_width*4),
        Square::F4 => (square_width*5,square_width*4),
        Square::G4 => (square_width*6,square_width*4),
        Square::H4 => (square_width*7,square_width*4),
        Square::A5 => (0,square_width*3),
        Square::B5 => (square_width*1,square_width*3),
        Square::C5 => (square_width*2,square_width*3),
        Square::D5 => (square_width*3,square_width*3),
        Square::E5 => (square_width*4,square_width*3),
        Square::F5 => (square_width*5,square_width*3),
        Square::G5 => (square_width*6,square_width*3),
        Square::H5 => (square_width*7,square_width*3),
        Square::A6 => (0,square_width*2),
        Square::B6 => (square_width*1,square_width*2),
        Square::C6 => (square_width*2,square_width*2),
        Square::D6 => (square_width*3,square_width*2),
        Square::E6 => (square_width*4,square_width*2),
        Square::F6 => (square_width*5,square_width*2),
        Square::G6 => (square_width*6,square_width*2),
        Square::H6 => (square_width*7,square_width*2),
        Square::A7 => (0,square_width*1),
        Square::B7 => (square_width*1,square_width*1),
        Square::C7 => (square_width*2,square_width*1),
        Square::D7 => (square_width*3,square_width*1),
        Square::E7 => (square_width*4,square_width*1),
        Square::F7 => (square_width*5,square_width*1),
        Square::G7 => (square_width*6,square_width*1),
        Square::H7 => (square_width*7,square_width*1),
        Square::A8 => (0,square_width*1),
        Square::B8 => (square_width*1,0),
        Square::C8 => (square_width*2,0),
        Square::D8 => (square_width*3,0),
        Square::E8 => (square_width*4,0),
        Square::F8 => (square_width*5,0),
        Square::G8 => (square_width*6,0),
        Square::H8 => (square_width*7,0)
    
    }
}
fn choose_piece_svg(piece: &PieceType) -> &'static str {
    match piece {
        PieceType::Pawn(Colour::White) => W_P,
        PieceType::Pawn(Colour::Black) => B_P,
        PieceType::King(Colour::White) => W_K,
        PieceType::King(Colour::Black) => B_K,
        PieceType::Queen(Colour::White) => W_Q,
        PieceType::Queen(Colour::Black) => B_Q,
        PieceType::Bishop(Colour::White) => W_B,
        PieceType::Bishop(Colour::Black) => B_B,
        PieceType::Knight(Colour::White) => W_N,
        PieceType::Knight(Colour::Black) => B_N,
        PieceType::Rook(Colour::White) => W_R,
        PieceType::Rook(Colour::Black) => B_R,
    
    }
}


const B_N : &str= r##"<path d="M22 10c10.5 1 16.5 8 16 29H15c0-9 10-6.5 8-21" fill="#000"/><path d="M24 18c.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.042-.94 1.41-3.04 0-3-1 0 .19 1.23-1 2-1 0-4.003 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-.73-.994-.5-2-.5-3 1-1 3 2.5 3 2.5h2s.78-1.992 2.5-3c1 0 1 3 1 3" fill="#000"/><path d="M9.5 25.5a.5.5 0 1 1-1 0 .5.5 0 1 1 1 0zm5.433-9.75a.5 1.5 30 1 1-.866-.5.5 1.5 30 1 1 .866.5z" fill="#ececec" stroke="#ececec"/><path d="M24.55 10.4l-.45 1.45.5.15c3.15 1 5.65 2.49 7.9 6.75S35.75 29.06 35.25 39l-.05.5h2.25l.05-.5c.5-10.06-.88-16.85-3.25-21.34-2.37-4.49-5.79-6.64-9.19-7.16l-.51-.1z" fill="#ececec" stroke="none"/>"##;
const B_K : &str= r##"<path d="M22.5 11.63V6" stroke-linejoin="miter"/><path d="M22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5" fill="#000" stroke-linecap="butt" stroke-linejoin="miter"/><path d="M11.5 37c5.5 3.5 15.5 3.5 21 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4V27v-3.5c-3.5-7.5-13-10.5-16-4-3 6 5 10 5 10V37z" fill="#000"/><path d="M20 8h5" stroke-linejoin="miter"/><path d="M32 29.5s8.5-4 6.03-9.65C34.15 14 25 18 22.5 24.5l.01 2.1-.01-2.1C20 18 9.906 14 6.997 19.85c-2.497 5.65 4.853 9 4.853 9" stroke="#ececec"/><path d="M11.5 30c5.5-3 15.5-3 21 0m-21 3.5c5.5-3 15.5-3 21 0m-21 3.5c5.5-3 15.5-3 21 0" stroke="#ececec"/>"##;
const B_P : &str = r##"<path d="M22.5 9c-2.21 0-4 1.79-4 4 0 .89.29 1.71.78 2.38C17.33 16.5 16 18.59 16 21c0 2.03.94 3.84 2.41 5.03-3 1.06-7.41 5.55-7.41 13.47h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62.49-.67.78-1.49.78-2.38 0-2.21-1.79-4-4-4z" fill="#000"/>"##;
const B_B : &str = r##"<g fill="#000" stroke-linecap="butt"><path d="M9 36c3.39-.97 10.11.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65.54 3 2-.68.97-1.65.99-3 .5-3.39-.97-10.11.46-13.5-1-3.39 1.46-10.11.03-13.5 1-1.354.49-2.323.47-3-.5 1.354-1.94 3-2 3-2z"/><path d="M15 32c2.5 2.5 12.5 2.5 15 0 .5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-.5.5 0 2z"/><path d="M25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/></g><path d="M17.5 26h10M15 30h15m-7.5-14.5v5M20 18h5" stroke="#ececec" stroke-linejoin="miter"/>"##;
const B_Q : &str = r##"<g fill="#000"><g stroke="none"><circle cx="6" cy="12" r="2.75"/><circle cx="14" cy="9" r="2.75"/><circle cx="22.5" cy="8" r="2.75"/><circle cx="31" cy="9" r="2.75"/><circle cx="39" cy="12" r="2.75"/></g><path d="M9 26c8.5-1.5 21-1.5 27 0l2.5-12.5L31 25l-.3-14.1-5.2 13.6-3-14.5-3 14.5-5.2-13.6L14 25 6.5 13.5 9 26z" stroke-linecap="butt"/><path d="M9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 .5 3.5-1.5 1-1.5 2.5-1.5 2.5-1.5 1.5.5 2.5.5 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 .5-1.5-1-2.5-.5-2.5-.5-2 .5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z" stroke-linecap="butt"/><path d="M11 38.5a35 35 1 0 0 23 0" fill="none" stroke-linecap="butt"/><path d="M11 29a35 35 1 0 1 23 0m-21.5 2.5h20m-21 3a35 35 1 0 0 22 0m-23 3a35 35 1 0 0 24 0" fill="none" stroke="#ececec"/></g>"##;
const B_R : &str = r##"<g fill="#000"><path d="M9 39h27v-3H9v3zm3.5-7l1.5-2.5h17l1.5 2.5h-20zm-.5 4v-4h21v4H12z" stroke-linecap="butt"/><path d="M14 29.5v-13h17v13H14z" stroke-linecap="butt" stroke-linejoin="miter"/><path d="M14 16.5L11 14h23l-3 2.5H14zM11 14V9h4v2h5V9h5v2h5V9h4v5H11z" stroke-linecap="butt"/><path d="M12 35.5h21m-20-4h19m-18-2h17m-17-13h17M11 14h23" fill="none" stroke="#ececec" stroke-width="1" stroke-linejoin="miter"/></g>"##;
const W_N : &str = r##"<path d="M22 10c10.5 1 16.5 8 16 29H15c0-9 10-6.5 8-21" fill="#fff"/><path d="M24 18c.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.042-.94 1.41-3.04 0-3-1 0 .19 1.23-1 2-1 0-4.003 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-.73-.994-.5-2-.5-3 1-1 3 2.5 3 2.5h2s.78-1.992 2.5-3c1 0 1 3 1 3" fill="#fff"/><path d="M9.5 25.5a.5.5 0 1 1-1 0 .5.5 0 1 1 1 0zm5.433-9.75a.5 1.5 30 1 1-.866-.5.5 1.5 30 1 1 .866.5z" fill="#000"/>"##;
const W_K : &str = r##"<path d="M22.5 11.63V6M20 8h5" stroke-linejoin="miter"/><path d="M22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5" fill="#fff" stroke-linecap="butt" stroke-linejoin="miter"/><path d="M11.5 37c5.5 3.5 15.5 3.5 21 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4V27v-3.5c-3.5-7.5-13-10.5-16-4-3 6 5 10 5 10V37z" fill="#fff"/><path d="M11.5 30c5.5-3 15.5-3 21 0m-21 3.5c5.5-3 15.5-3 21 0m-21 3.5c5.5-3 15.5-3 21 0"/>"##;
const W_P : &str = r##"<path d="M22.5 9c-2.21 0-4 1.79-4 4 0 .89.29 1.71.78 2.38C17.33 16.5 16 18.59 16 21c0 2.03.94 3.84 2.41 5.03-3 1.06-7.41 5.55-7.41 13.47h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62.49-.67.78-1.49.78-2.38 0-2.21-1.79-4-4-4z" fill="#fff"/>"##;
const W_B : &str = r##"<g fill="#fff" stroke-linecap="butt"><path d="M9 36c3.39-.97 10.11.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65.54 3 2-.68.97-1.65.99-3 .5-3.39-.97-10.11.46-13.5-1-3.39 1.46-10.11.03-13.5 1-1.354.49-2.323.47-3-.5 1.354-1.94 3-2 3-2z"/><path d="M15 32c2.5 2.5 12.5 2.5 15 0 .5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-.5.5 0 2z"/><path d="M25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/></g><path d="M17.5 26h10M15 30h15m-7.5-14.5v5M20 18h5" stroke-linejoin="miter"/>"##;
const W_Q : &str = r##"<g fill="#fff"><path d="M8 12a2 2 0 1 1-4 0 2 2 0 1 1 4 0zm16.5-4.5a2 2 0 1 1-4 0 2 2 0 1 1 4 0zM41 12a2 2 0 1 1-4 0 2 2 0 1 1 4 0zM16 8.5a2 2 0 1 1-4 0 2 2 0 1 1 4 0zM33 9a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/><path d="M9 26c8.5-1.5 21-1.5 27 0l2-12-7 11V11l-5.5 13.5-3-15-3 15-5.5-14V25L7 14l2 12z" stroke-linecap="butt"/><path d="M9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 .5 3.5-1.5 1-1.5 2.5-1.5 2.5-1.5 1.5.5 2.5.5 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 .5-1.5-1-2.5-.5-2.5-.5-2 .5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z" stroke-linecap="butt"/><path d="M11.5 30c3.5-1 18.5-1 22 0M12 33.5c6-1 15-1 21 0" fill="none"/></g>"##;
const W_R : &str = r##"<g fill="#fff"><path d="M9 39h27v-3H9v3zm3-3v-4h21v4H12zm-1-22V9h4v2h5V9h5v2h5V9h4v5" stroke-linecap="butt"/><path d="M34 14l-3 3H14l-3-3"/><path d="M31 17v12.5H14V17" stroke-linecap="butt" stroke-linejoin="miter"/><path d="M31 29.5l1.5 2.5h-20l1.5-2.5"/><path d="M11 14h23" fill="none" stroke-linejoin="miter"/></g>"##;