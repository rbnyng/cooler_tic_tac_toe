use eframe::egui;
use std::collections::VecDeque;

struct TicTacToeApp {
    board: Vec<Option<char>>,
    player_turn: char,
    moves_history: VecDeque<(usize, char)>,
    winner: Option<char>,
    dark_mode: bool,
}

impl Default for TicTacToeApp {
    fn default() -> Self {
        Self {
            board: vec![None; 9],
            player_turn: 'X',
            moves_history: VecDeque::new(),
            winner: None,
            dark_mode: false,
        }
    }
}

impl eframe::App for TicTacToeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Toggle between dark mode and light mode
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Dark Mode");
                ui.checkbox(&mut self.dark_mode, "");
            });
        });

        // Set visual style based on dark mode
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(winner) = self.winner {
                ui.heading(format!("Player {} wins!", winner));
                if ui.button("Restart").clicked() {
                    self.reset_game();
                }
            } else {
                ui.heading("Tic Tac Toe but cooler");

                let (response, painter) = ui.allocate_painter(egui::Vec2::new(300.0, 300.0), egui::Sense::click());

                let rect = response.rect;
                let grid_size = rect.size() / 3.0;
                let line_color = if self.dark_mode { egui::Color32::WHITE } else { egui::Color32::BLACK };
                let piece_color = if self.dark_mode { egui::Color32::WHITE } else { egui::Color32::BLACK };

                for i in 0..3 {
                    for j in 0..3 {
                        let cell_rect = egui::Rect::from_min_max(
                            rect.min + egui::vec2(i as f32 * grid_size.x, j as f32 * grid_size.y),
                            rect.min + egui::vec2((i + 1) as f32 * grid_size.x, (j + 1) as f32 * grid_size.y),
                        );

                        painter.rect_stroke(cell_rect, 0.0, (2.0, line_color));

                        if let Some(player) = self.board[i + j * 3] {
                            painter.text(
                                cell_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                player,
                                egui::FontId::proportional(64.0),
                                piece_color,
                            );
                        }

                        if response.clicked() && cell_rect.contains(response.interact_pointer_pos().unwrap()) {
                            self.handle_move(i + j * 3);
                        }
                    }
                }
            }
        });
    }
}

impl TicTacToeApp {
    fn switch_player(&mut self) {
        self.player_turn = if self.player_turn == 'X' { 'O' } else { 'X' };
    }

    fn handle_move(&mut self, idx: usize) {
        if self.board[idx].is_none() && self.winner.is_none() {
            self.board[idx] = Some(self.player_turn);
            self.moves_history.push_back((idx, self.player_turn));
            
            if self.check_winner() {
                self.winner = Some(self.player_turn);
            } else if self.check_draw() {
                self.winner = Some('D'); // 'D' for Draw
            } else {
                self.switch_player();
            }
        }
    }

    fn check_winner(&self) -> bool {
        let b = &self.board;
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
            [0, 4, 8], [2, 4, 6] // diagonals
        ];
        
        lines.iter().any(|&line| {
            line.iter().all(|&i| b[i] == Some(self.player_turn))
        })
    }

    fn check_draw(&self) -> bool {
        self.board.iter().all(|&cell| cell.is_some())
    }

    fn reset_game(&mut self) {
        self.board.fill(None);
        self.moves_history.clear();
        self.winner = None;
        self.player_turn = 'X';
    }
}

// native app
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Tic Tac Toe",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    );
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                options,
                Box::new(|_cc| Box::new(TicTacToeApp::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
