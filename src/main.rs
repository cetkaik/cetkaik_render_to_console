use cetkaik_compact_representation::{Field, Coord};
use cetkaik_fundamental::AbsoluteSide;
use cetkaik_traits::{IsAbsoluteField, IsField};
use cetkaik_render_to_console::*;

fn main() {
    let mut field = Field::yhuap_initial();
    field.print_to_console();
    field = field
        .move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
            Coord::new(8, 8).unwrap(),
            Coord::new(0, 0).unwrap(),
            AbsoluteSide::IASide,
        )
        .unwrap();
    field.print_to_console();
}
