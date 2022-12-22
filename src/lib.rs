#![warn(clippy::pedantic)]
use cetkaik_compact_representation::CetkaikCompact;
use cetkaik_fundamental::{serialize_prof, AbsoluteSide, Color};
use cetkaik_naive_representation::{
    perspective::{to_relative_board, Perspective},
    relative, CetkaikNaive,
};
use cetkaik_traits::{CetkaikRepresentation, IsField};
use colored::Colorize;
use relative::Piece;

/// Using `cetkaik_compact_representation`:
/// ```
/// use cetkaik_compact_representation::{Field, Coord};
/// use cetkaik_fundamental::AbsoluteSide;
/// use cetkaik_traits::{IsAbsoluteField, IsField};
/// use cetkaik_render_to_console::*;
/// let mut field = Field::yhuap_initial();
/// field.print_to_console();
/// field = field
///     .move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
///         Coord::new(8, 8).unwrap(),
///         Coord::new(0, 0).unwrap(),
///         AbsoluteSide::IASide,
///     )
///     .unwrap();
/// field.print_to_console();
/// ```
pub trait PrintToConsole {
    fn to_colored_string(&self) -> String;
    fn print_to_console(&self) {
        print!("{}", self.to_colored_string());
    }
}

impl PrintToConsole for relative::Piece {
    fn to_colored_string(&self) -> String {
        match *self {
            Piece::Tam2 => format!("-{}", "皇".bold()),
            Piece::NonTam2Piece {
                color,
                prof,
                side: relative::Side::Downward,
            } => {
                if color == Color::Huok2 {
                    format!("v{}", serialize_prof(prof)).bold().to_string()
                } else {
                    format!("v{}", serialize_prof(prof))
                        .bold()
                        .red()
                        .to_string()
                }
            }
            Piece::NonTam2Piece {
                color,
                prof,
                side: relative::Side::Upward,
            } => {
                if color == Color::Huok2 {
                    format!("^{}", serialize_prof(prof)).bold().to_string()
                } else {
                    format!("^{}", serialize_prof(prof))
                        .bold()
                        .red()
                        .to_string()
                }
            }
        }
    }
}

impl PrintToConsole for cetkaik_naive_representation::absolute::Board {
    fn to_colored_string(&self) -> String {
        absolute_board_to_colored_string(
            to_relative_board(self, Perspective::IaIsDownAndPointsUpward).0,
            &|piece| piece,
        )
    }
}

fn absolute_board_to_colored_string<T>(
    board: [[Option<T>; 9]; 9],
    to_relative: &dyn Fn(T) -> relative::Piece,
) -> String {
    format!(
        "-{}-\n{}",
        vec!["K", "L", "N", "T", "Z", "X", "C", "M", "P"].join("一-"),
        board
            .into_iter()
            .enumerate()
            .map(|(index, row)| {
                format!(
                    "{} {}\n",
                    row.into_iter()
                        .map(|p| {
                            match p {
                                None => "-一".to_string(),
                                Some(piece) => to_relative(piece).to_colored_string(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("|"),
                    [" A", " E", " I", " U", " O", " Y", "AI", "AU", "IA"][index]
                )
            })
            .collect::<String>()
    )
}

impl PrintToConsole for cetkaik_compact_representation::Board {
    fn to_colored_string(&self) -> String {
        absolute_board_to_colored_string(self.to_piece_array(), &|piece| {
            use cetkaik_compact_representation::MaybeTam2;
            match piece.prof_and_side() {
                MaybeTam2::Tam2 => relative::Piece::Tam2,
                MaybeTam2::NotTam2((prof, side)) => relative::Piece::NonTam2Piece {
                    color: piece.color(),
                    prof,
                    side: match side {
                        AbsoluteSide::ASide => relative::Side::Downward,
                        AbsoluteSide::IASide => relative::Side::Upward,
                    },
                },
            }
        })
    }
}

fn absolute_field_to_colored_string<T: CetkaikRepresentation>(field: &T::AbsoluteField) -> String
where
    T::AbsoluteBoard: PrintToConsole,
{
    format!(
        "ASide hop1zuo1: [{}]\n\n{}\n\nIASide hop1zuo1: [{}]\n\n======================================\n\n",
        T::hop1zuo1_of(AbsoluteSide::ASide, field) .into_iter()
            .map(|cp| relative::Piece::NonTam2Piece {
                color: cp.color,
                prof: cp.prof,
                side: relative::Side::Downward
            }
            .to_colored_string())
            .collect::<Vec<_>>()
            .join(" "),
        field.as_board().to_colored_string(),
        T::hop1zuo1_of(AbsoluteSide::IASide, field) .into_iter()
            .map(|cp| relative::Piece::NonTam2Piece {
                color: cp.color,
                prof: cp.prof,
                side: relative::Side::Upward
            }
            .to_colored_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

impl PrintToConsole for cetkaik_compact_representation::Field {
    fn to_colored_string(&self) -> String {
        absolute_field_to_colored_string::<CetkaikCompact>(self)
    }
}

impl PrintToConsole for cetkaik_naive_representation::absolute::Field {
    fn to_colored_string(&self) -> String {
        absolute_field_to_colored_string::<CetkaikNaive>(self)
    }
}
