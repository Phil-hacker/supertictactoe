use borsh::{BorshDeserialize, BorshSerialize};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize,
)]
pub enum Field {
    Cross,
    Circle,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub enum SmallGame {
    Unfinished([Field; 9]),
    Cross,
    Circle,
    Draw,
}

static WIN_CONDITIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl SmallGame {
    pub fn is_finished(&self) -> bool {
        !matches!(self, SmallGame::Unfinished(_))
    }
    pub fn update(self) -> Self {
        match self {
            SmallGame::Unfinished(game) => {
                for condition in WIN_CONDITIONS {
                    if game[condition[0]] == game[condition[1]]
                        && game[condition[1]] == game[condition[2]]
                        && game[condition[0]] == Field::Cross
                    {
                        return SmallGame::Cross;
                    }
                }
                for condition in WIN_CONDITIONS {
                    if game[condition[0]] == game[condition[1]]
                        && game[condition[1]] == game[condition[2]]
                        && game[condition[0]] == Field::Circle
                    {
                        return SmallGame::Circle;
                    }
                }
                if !game.iter().any(|v| *v == Field::None) {
                    return Self::Draw;
                }
                self
            }
            SmallGame::Cross => self,
            SmallGame::Circle => self,
            SmallGame::Draw => self,
        }
    }
}

impl Default for SmallGame {
    fn default() -> Self {
        Self::Unfinished([Field::default(); 9])
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub enum Player {
    #[default]
    Cross,
    Circle,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Move {
    player: Player,
    field: usize,
}

impl Move {
    pub fn new(player: Player, field: usize) -> Self {
        Move { player, field }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub enum NextMove {
    #[default]
    Everywhere,
    Field(usize),
}
#[derive(Debug, Default, Clone, Copy)]
pub enum Winner {
    Draw,
    Cross,
    Circle,
    #[default]
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub struct Game {
    field: [SmallGame; 9],
    next_move: NextMove,
    next_player: Player,
}

impl Game {
    pub fn get_field(&self) -> &[SmallGame; 9] {
        &self.field
    }
    pub fn get_next_move(&self) -> NextMove {
        self.next_move
    }
    pub fn get_next_player(&self) -> Player {
        self.next_player
    }
    pub fn get_finished_fields(&self) -> [bool; 9] {
        self.field.map(|v| v.is_finished())
    }
    pub fn play(&mut self, play: Move) -> Result<(), ()> {
        if play.player != self.next_player {
            return Err(());
        }
        let field = play.field / 9;
        if let NextMove::Field(next_move) = self.next_move {
            if next_move != field {
                return Err(());
            }
        }
        if let SmallGame::Unfinished(smallgame) = &mut self.field[field] {
            if smallgame[play.field % 9] == Field::None {
                (self.next_player, smallgame[play.field % 9]) = {
                    match self.next_player {
                        Player::Cross => (Player::Circle, Field::Cross),
                        Player::Circle => (Player::Cross, Field::Circle),
                    }
                };
            } else {
                return Err(());
            }
        }
        self.field[field] = self.field[field].update();
        if matches!(self.field[play.field % 9], SmallGame::Unfinished(_)) {
            self.next_move = NextMove::Field(play.field % 9)
        } else {
            self.next_move = NextMove::Everywhere
        }
        Ok(())
    }
    pub fn get_winner(&self) -> Winner {
        let game = self.field;
        for condition in WIN_CONDITIONS {
            if game[condition[0]] == game[condition[1]]
                && game[condition[1]] == game[condition[2]]
                && game[condition[0]] == SmallGame::Cross
            {
                return Winner::Cross;
            }
        }
        for condition in WIN_CONDITIONS {
            if game[condition[0]] == game[condition[1]]
                && game[condition[1]] == game[condition[2]]
                && game[condition[0]] == SmallGame::Circle
            {
                return Winner::Circle;
            }
        }
        if !game.iter().any(|v| matches!(*v, SmallGame::Unfinished(_))) {
            return Winner::Draw;
        }
        Winner::None
    }
}
