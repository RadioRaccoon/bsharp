use std::{io, env, fs};

use shakmaty::{CastlingMode, Chess, Position};
use shakmaty::fen::Fen;

use pgn_reader::{Visitor, Skip, RawHeader, BufferedReader, SanPlus};

struct LastPosition {
    pos: Chess,
}

impl LastPosition {
    fn new() -> LastPosition {
        LastPosition { pos: Chess::default() }
    }
}

impl Visitor for LastPosition {
    type Result = Chess;

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
        if key == b"FEN" {
            let pos = Fen::from_ascii(value.as_bytes()).ok()
                .and_then(|f| f.into_position(CastlingMode::Standard).ok());

            if let Some(pos) = pos {
                self.pos = pos;
            }
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn san(&mut self, san_plus: SanPlus) {
        if let Ok(m) = san_plus.san.to_move(&self.pos) {
            self.pos.play_unchecked(&m);
        }
    }

    fn end_game(&mut self) -> Self::Result {
        ::std::mem::replace(&mut self.pos, Chess::default())
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let pgn = b"1. f3 e5 2. g4 Qh4#";

    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    let mut visitor = LastPosition::new();
    let pos = reader.read_game(&mut visitor)?;

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    assert!(pos.map_or(false, |p| p.is_checkmate()));
    Ok(())
}
