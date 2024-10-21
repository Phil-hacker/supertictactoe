use egui::{Painter, Pos2, Rect, Sense, Stroke, Vec2, Widget};

use crate::game::*;

#[derive(Debug)]
pub struct GameField<'a> {
    pub games: &'a mut Vec<Game>,
    pub player: Option<Player>,
}

impl<'a> GameField<'a> {
    pub fn new(games: &'a mut Vec<Game>, player: Option<Player>) -> Self {
        Self {
            games,
            player,
        }
    }
}

impl Widget for GameField<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = (ui.available_size_before_wrap() - Vec2::new(0.0, 100.0))
            .min(Vec2 {
                x: 1000.0,
                y: 1000.0,
            })
            .min_elem();
        let (bound, response) = ui.allocate_exact_size(Vec2 { x: size, y: size }, Sense::click());
        let size = bound.size();
        let length = size.min_elem();
        let stroke = ui.visuals().noninteractive().fg_stroke;
        let painter = ui.painter();
        let mut game = self.games.last().unwrap().clone();
        let (ubergrids, grids) = draw_full_grid(
            painter,
            bound.min,
            length,
            stroke,
            &game.get_finished_fields(),
            game.get_next_move(),
        );
        for (i, game) in game.get_field().iter().enumerate() {
            match game {
                SmallGame::Unfinished(small_game) => {
                    for (j, field) in small_game.iter().enumerate() {
                        match field {
                            Field::Cross => {
                                draw_cross(painter, stroke, grids[i * 9 + j], length / 100.0)
                            }
                            Field::Circle => {
                                draw_circle(painter, stroke, grids[i * 9 + j], length / 100.0)
                            }
                            Field::None => {}
                        }
                    }
                }
                SmallGame::Cross => draw_cross(painter, stroke, ubergrids[i], length / 30.0),
                SmallGame::Circle => draw_circle(painter, stroke, ubergrids[i], length / 30.0),
                SmallGame::Draw => draw_none(painter, stroke, ubergrids[i], length / 30.0),
            }
        }
        if response.clicked() {
            if let Some(click_pos) = response.interact_pointer_pos() {
                let grids = grids.map(|grid| grid.contains(click_pos));
                let pos = grids.iter().position(|v| *v);
                if let Some(pos) = pos {
                    println!("{pos}");
                    let game_move = Move::new(
                        self.player.unwrap_or_else(|| game.get_next_player()),
                        pos,
                    );
                    if game.play(game_move).is_ok() {
                        self.games.push(game)
                    }
                }
            }
        }
        response
    }
}

fn draw_full_grid(
    painter: &Painter,
    starting_point: Pos2,
    size: f32,
    stroke: Stroke,
    disabled: &[bool; 9],
    highlight: NextMove,
) -> ([Rect; 9], [Rect; 81]) {
    let segment_length = size / 17.0;
    let grids = draw_grid(painter, starting_point, size, segment_length, stroke, true);
    let ubergrids = grids;
    let grids = grids.into_iter().enumerate();
    let grids: [Rect; 81] = {
        let grid: Vec<Rect> = grids
            .flat_map(|(i, grid)| {
                let highlighted = match highlight {
                    NextMove::Everywhere => true,
                    NextMove::Field(field) => i == field,
                };
                if !disabled[i] {
                    draw_grid(
                        painter,
                        grid.min,
                        grid.height(),
                        segment_length / 2.0,
                        stroke,
                        highlighted,
                    )
                } else {
                    [Rect::NOTHING; 9]
                }
            })
            .collect();
        grid.try_into().unwrap()
    };
    (ubergrids, grids)
}

fn draw_grid(
    painter: &Painter,
    starting_point: Pos2,
    size: f32,
    margin: f32,
    mut stroke: Stroke,
    highlight: bool,
) -> [Rect; 9] {
    let segment_length = (size - margin * 2.0) / 3.0;
    let starting_point = starting_point + Vec2::new(margin, margin);
    if highlight {
        stroke = Stroke {
            width: stroke.width * 3.0,
            ..stroke
        }
    }
    painter.line_segment(
        [
            starting_point + Vec2::new(0.0, segment_length),
            starting_point + Vec2::new(segment_length * 3.0, segment_length),
        ],
        stroke,
    );
    painter.line_segment(
        [
            starting_point + Vec2::new(0.0, segment_length * 2.0),
            starting_point + Vec2::new(segment_length * 3.0, segment_length * 2.0),
        ],
        stroke,
    );
    painter.line_segment(
        [
            starting_point + Vec2::new(segment_length, 0.0),
            starting_point + Vec2::new(segment_length, segment_length * 3.0),
        ],
        stroke,
    );
    painter.line_segment(
        [
            starting_point + Vec2::new(segment_length * 2.0, 0.0),
            starting_point + Vec2::new(segment_length * 2.0, segment_length * 3.0),
        ],
        stroke,
    );
    [
        Rect {
            min: starting_point,
            max: starting_point + Vec2::new(segment_length, segment_length),
        },
        Rect {
            min: starting_point + Vec2::new(segment_length, 0.0),
            max: starting_point + Vec2::new(segment_length * 2.0, segment_length),
        },
        Rect {
            min: starting_point + Vec2::new(segment_length * 2.0, 0.0),
            max: starting_point + Vec2::new(segment_length * 3.0, segment_length),
        },
        Rect {
            min: starting_point + Vec2::new(0.0, segment_length),
            max: starting_point + Vec2::new(segment_length, segment_length * 2.0),
        },
        Rect {
            min: starting_point + Vec2::new(segment_length, segment_length),
            max: starting_point + Vec2::new(segment_length * 2.0, segment_length * 2.0),
        },
        Rect {
            min: starting_point + Vec2::new(segment_length * 2.0, segment_length),
            max: starting_point + Vec2::new(segment_length * 3.0, segment_length * 2.0),
        },
        Rect {
            min: starting_point + Vec2::new(0.0, segment_length * 2.0),
            max: starting_point + Vec2::new(segment_length, segment_length * 3.0),
        },
        Rect {
            min: starting_point + Vec2::new(segment_length, segment_length * 2.0),
            max: starting_point + Vec2::new(segment_length * 2.0, segment_length * 3.0),
        },
        Rect {
            min: starting_point + Vec2::new(segment_length * 2.0, segment_length * 2.0),
            max: starting_point + Vec2::new(segment_length * 3.0, segment_length * 3.0),
        },
    ]
}

fn draw_circle(painter: &Painter, stroke: Stroke, bounding_box: Rect, margin: f32) {
    let bounding_box = Rect {
        min: bounding_box.min + Vec2::new(margin, margin),
        max: bounding_box.max - Vec2::new(margin, margin),
    };
    painter.circle_stroke(bounding_box.center(), bounding_box.size().x / 2.0, stroke);
}

fn draw_cross(painter: &Painter, stroke: Stroke, bounding_box: Rect, margin: f32) {
    let bounding_box = Rect {
        min: bounding_box.min + Vec2::new(margin, margin),
        max: bounding_box.max - Vec2::new(margin, margin),
    };
    painter.line_segment(
        [bounding_box.left_bottom(), bounding_box.right_top()],
        stroke,
    );
    painter.line_segment(
        [bounding_box.left_top(), bounding_box.right_bottom()],
        stroke,
    );
}

fn draw_none(painter: &Painter, stroke: Stroke, bounding_box: Rect, margin: f32) {
    let bounding_box = Rect {
        min: bounding_box.min + Vec2::new(margin, margin),
        max: bounding_box.max - Vec2::new(margin, margin),
    };
    painter.rect_stroke(bounding_box, 0.0, stroke);
    painter.line_segment(
        [bounding_box.left_bottom(), bounding_box.right_top()],
        stroke,
    );
    painter.line_segment(
        [bounding_box.left_top(), bounding_box.right_bottom()],
        stroke,
    );
    painter.circle_stroke(bounding_box.center(), bounding_box.size().x / 2.0, stroke);
}
