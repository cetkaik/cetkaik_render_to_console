use cetkaik_compact_representation::*;
use cetkaik_fundamental::{serialize_prof, AbsoluteSide, Color};
use cetkaik_naive_representation::relative;
use cetkaik_traits::IsAbsoluteField;
use cetkaik_traits::IsField;
use colored::Colorize;
use relative::Piece;

trait PrintToConsole {
    fn to_colored_string(&self) -> String;
    fn print_to_console(&self) {
        print!("{}", self.to_colored_string())
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

impl PrintToConsole
    for (
        [Option<cetkaik_compact_representation::PieceWithSide>; 9],
        &str,
    )
{
    fn to_colored_string(&self) -> String {
        format!(
            "{} {}\n",
            self.0
                .into_iter()
                .map(|p| {
                    match p {
                        None => "-一".to_string(),
                        Some(piece) => match piece.prof_and_side() {
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
                        .to_colored_string(),
                    }
                })
                .collect::<Vec<_>>()
                .join("|"),
            self.1
        )
    }
}

impl PrintToConsole for Board {
    fn to_colored_string(&self) -> String {
        format!(
            "-{}-\n{}",
            vec!["K", "L", "N", "T", "Z", "X", "C", "M", "P"].join("一-"),
            self.to_piece_array()
                .into_iter()
                .enumerate()
                .map(|(index, row)| {
                    (
                        row,
                        [" A", " E", " I", " U", " O", " Y", "AI", "AU", "IA"][index],
                    )
                        .to_colored_string()
                })
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

fn print_absolute_field(field: &Field) {
    println!(
        "ASide hop1zuo1: [{}]\n",
        field
            .as_hop1zuo1()
            .a_side_hop1zuo1_color_and_prof()
            .map(|cp| relative::Piece::NonTam2Piece {
                color: cp.color,
                prof: cp.prof,
                side: relative::Side::Downward
            }
            .to_colored_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    field.as_board().print_to_console();
    println!(
        "\nIASide hop1zuo1: [{}]\n",
        field
            .as_hop1zuo1()
            .ia_side_hop1zuo1_color_and_prof()
            .map(|cp| relative::Piece::NonTam2Piece {
                color: cp.color,
                prof: cp.prof,
                side: relative::Side::Upward
            }
            .to_colored_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("======================================\n\n")
}

fn main() {
    let mut field = Field::yhuap_initial();
    print_absolute_field(&field);
    field = field
        .move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
            Coord::new(8, 8).unwrap(),
            Coord::new(0, 0).unwrap(),
            AbsoluteSide::IASide,
        )
        .unwrap();
    print_absolute_field(&field);

    /*println!(
        "{}, {}, {}, {}, {}, {}, and some normal text.\n {}",
        format!("Bold").bold(),
        format!("Red").red(),
        format!("Yellow").yellow(),
        format!("Green Strikethrough").green().strikethrough(),
        format!("Blue Underline").blue().underline(),
        format!("Purple Italics").purple().italic(),
        "bright colors are also allowed"
            .bright_blue()
            .on_bright_white()
    );*/
}
