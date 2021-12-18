
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
