use cetkaik_fundamental::AbsoluteSide;
use cetkaik_naive_representation::absolute::{Column, Coord, Field, Row};
use cetkaik_render_to_console::*;
use cetkaik_traits::{IsAbsoluteField, IsField};

fn main() {
    let mut field = Field::yhuap_initial();
    field.print_to_console();
    field = field
        .move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
            Coord(Row::IA, Column::P),
            Coord(Row::A, Column::K),
            AbsoluteSide::IASide,
        )
        .unwrap();
    field.print_to_console();
}
