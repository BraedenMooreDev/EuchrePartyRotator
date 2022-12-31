#![allow(non_snake_case)]
#![allow(unused)]
use egui::{Widget, Vec2, Frame};
use rand::Rng;
use rand::rngs::mock::StepRng;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:

    // this how you opt-out of serialization of a member
    playerCount: usize,
    tableCount: usize,
    outCount: usize,
    gameCount: usize,
    separator: String,
    displayNames: bool,
    cardData: Vec<(Vec<((usize, usize), (usize, usize))>, Vec<usize>)>,
    playerNames: Vec<String>,
    #[serde(skip)]
    fontSettingsOpen: bool,
    font_id: egui::FontId,
    gridSpacing: Vec2,
    max_col_width: f32,
    col_spacer: i32,
    background_color: egui::Color32
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            playerCount: 12,
            tableCount: 3,
            outCount: 0,
            gameCount: 11,
            separator: String::from("-"),
            displayNames: false,
            cardData: shuffle(12, 3, 11, 0),
            playerNames: Vec::new(),
            fontSettingsOpen: false,
            font_id: egui::FontId::default(),
            gridSpacing: Vec2::new(2.0, 2.0),
            max_col_width: 20.0,
            col_spacer: 10,
            background_color: egui::Color32::from_rgb(28, 28, 28)
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { playerCount , tableCount, outCount, gameCount, separator, displayNames, cardData, playerNames, fontSettingsOpen: settingsOpen, font_id, gridSpacing , max_col_width, col_spacer, background_color} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if false {
            #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Settings", |ui| {

                        ui.add(egui::Separator::default());

                        if ui.button("Settings").clicked() {

                            *settingsOpen = !*settingsOpen;
                        }
                    });
                });
            });
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.heading("Euchre Party Rotator");
                ui.add_space(10.0);
                ui.horizontal(|ui| {

                    if ui.button("Clear Names").clicked() {

                        *playerNames = Vec::new();
                    }

                    if ui.button("Settings").clicked() {

                        *settingsOpen = !*settingsOpen;
                    }
                });

                ui.add_space(8.0);

                if ui.add(egui::Slider::new(playerCount, 4..=50).integer()).changed() {

                    playerNames.shrink_to(*playerCount);

                    *tableCount = *playerCount / 4;
                    *outCount = *playerCount % 4;

                    if *outCount == 0 {
                        *gameCount = *playerCount - 1;
                    } else {
                        *gameCount = *playerCount;
                    }

                    *cardData = shuffle(*playerCount, *tableCount, *gameCount, *outCount);
                }
                
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    
                    for playerId in 0..*playerCount {

                        if playerNames.len() <= playerId {
                            playerNames.push("".to_owned());
                        }
        
                        ui.horizontal(|ui| {
                            ui.label(format!("Player {}", playerId + 1));
                            egui::TextEdit::singleline(&mut playerNames[playerId])
                                .hint_text("Name").ui(ui)
                                .context_menu(|ui| {
                                    if ui.button("Insert Before").clicked() {
                                        *playerCount += 1;
                                        playerNames.insert(playerId, "".to_owned());
                        
                                        *tableCount = *playerCount / 4;
                                        *outCount = *playerCount % 4;
                        
                                        if *outCount == 0 {
                                            *gameCount = *playerCount - 1;
                                        } else {
                                            *gameCount = *playerCount;
                                        }
                        
                                        *cardData = shuffle(*playerCount, *tableCount, *gameCount, *outCount);
                                    }
                                    if ui.button("Insert After").clicked() {
                                        *playerCount += 1;
                                        playerNames.insert(playerId + 1, "".to_owned());

                                        *tableCount = *playerCount / 4;
                                        *outCount = *playerCount % 4;
                        
                                        if *outCount == 0 {
                                            *gameCount = *playerCount - 1;
                                        } else {
                                            *gameCount = *playerCount;
                                        }
                        
                                        *cardData = shuffle(*playerCount, *tableCount, *gameCount, *outCount);
                                    }
                                    ui.add(egui::Separator::default());
                                    if ui.button("Delete").clicked() {

                                        if *playerCount == 4 {
                                            return;
                                        }

                                        *playerCount -= 1;
                                        playerNames.remove(playerId);

                                        *tableCount = *playerCount / 4;
                                        *outCount = *playerCount % 4;
                        
                                        if *outCount == 0 {
                                            *gameCount = *playerCount - 1;
                                        } else {
                                            *gameCount = *playerCount;
                                        }
                        
                                        *cardData = shuffle(*playerCount, *tableCount, *gameCount, *outCount);
                                    }
                                });
                        });
                    }
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.hyperlink("https://github.com/BraedenMooreDev");
                ui.hyperlink("https://bmooredev.weebly.com");
                ui.label("Created by Braeden Moore");
                egui::warn_if_debug_build(ui);
            });
        });

        egui::Window::new("🗖 Settings")
            .resizable(false)
            .open(settingsOpen)
            .show(ctx, |ui| {

                ui.heading("Display");
                ui.add_space(5.0);

                if ui.button("Toggle Light/Dark Mode").clicked() {

                    let visuals = if ui.visuals().dark_mode {
                        egui::Visuals::light()
                    } else {
                        egui::Visuals::dark()
                    };

                    ctx.set_visuals(visuals);
                }

                ui.horizontal(|ui| {
                    ui.color_edit_button_srgba(&mut *background_color);
                    ui.label("Background Color");
                });
                ui.separator();
                

                ui.heading("Font");
                ui.add_space(5.0);
                egui::introspection::font_id_ui(ui, &mut self.font_id);

                ui.separator();

                ui.heading("Table Layout");
                ui.add_space(5.0);
                ui.add(egui::Slider::new(&mut gridSpacing.x, 0.0..=100.0).text("Horizontal Spacing"));
                ui.add(egui::Slider::new(&mut gridSpacing.y, 0.0..=50.0).text("Vertical Spacing"));

                ui.separator();

                ui.heading("Formatting");
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Separator");
                    ui.text_edit_singleline(separator)
                        .context_menu(|ui| {
                            if ui.button("Set to Default").clicked() {
                                *separator = String::from("-");
                            }
                        });
                });
                ui.add(egui::Checkbox::new(displayNames, "Display Names"));
        });

        egui::CentralPanel::default()
            .frame(Frame::none()
                    .fill(*background_color).inner_margin(10.0))
            .show(ctx, |ui| {

            egui::ScrollArea::both().show(ui, |ui| {

                egui::Grid::new("EuchreRotationCard")
                .striped(true)
                .spacing(*gridSpacing)
                .min_row_height(0.0)
                .min_col_width(10.0)
                .show(ui, |ui| {

                        for row in 0..=*gameCount {
                                for col in 0..=*tableCount {

                                    match row {
                                        0 => {
                                            match col {
                                                0 => {
                                                    ui.label(egui::RichText::new("Game").font(self.font_id.clone()).strong().underline());
                                                }
                                                _ => {
                                                    ui.add(egui::Separator::vertical(egui::Separator::default()));
                                                    ui.label("");
                                                    ui.label(egui::RichText::new(format!("Table {}", col)).font(self.font_id.clone()).strong().underline());
                                                    ui.label("");
                                                }
                                            }
                                        }
                                        _ => {
                                            match col {
                                                0 => {
                                                    ui.label(egui::RichText::new(format!("{}", row)).font(self.font_id.clone()).strong());
                                                }
                                                _ => {
                                                    ui.add(egui::Separator::vertical(egui::Separator::default()));
                                                    ui.label(egui::RichText::new(format!("{}", formatPlayersTuple(cardData[row - 1].0[col - 1].0, separator.clone(), *displayNames, playerNames.clone()))).font(self.font_id.clone()));
                                                    ui.centered_and_justified(|ui| {
                                                        ui.label(egui::RichText::new("vs").font(self.font_id.clone()).weak())});
                                                    ui.label(egui::RichText::new(format!("{}", formatPlayersTuple(cardData[row - 1].0[col - 1].1, separator.clone(), *displayNames, playerNames.clone()))).font(self.font_id.clone()));
                                                }
                                            }
                                        }
                                    }
                                }

                                if *playerCount % 4 != 0 {

                                    ui.add_space(5.0);
                                    match row {
                                        0 => {
                                            ui.add(egui::Separator::vertical(egui::Separator::default()));
                                            ui.label(egui::RichText::new("Out").font(self.font_id.clone()).strong().underline());
                                            ui.add(egui::Separator::vertical(egui::Separator::default()));
                                        }
                                        _ => {
                                            ui.add(egui::Separator::vertical(egui::Separator::default()));
                                            ui.label(egui::RichText::new(format!("{}", formatPlayersVector(cardData[row - 1].1.clone(), separator.clone(), *displayNames, playerNames.clone()))).font(self.font_id.clone()));
                                            ui.add(egui::Separator::vertical(egui::Separator::default()));
                                        }
                                    }
                                }

                            ui.end_row();
                        }
                });
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

fn shuffle(playerCount: usize, tableCount: usize, gameCount: usize, outCount: usize) -> Vec<(Vec<((usize, usize), (usize, usize))>, Vec<usize>)> {

    let mut full: Vec<(Vec<((usize, usize), (usize, usize))>, Vec<usize>)> = Vec::with_capacity(gameCount);

    let mut game: Vec<((usize, usize), (usize, usize))> = Vec::with_capacity(tableCount);
    let mut out: Vec<usize> = Vec::with_capacity(outCount);

    let mut players: Vec<usize>;
    let mut usedPartners: Vec<(usize, usize)> = Vec::new();
    let mut usedNeighbors: Vec<(usize, usize)> = Vec::new(); 

    for gameIndex in 0..gameCount {

        game.clear();
        out.clear();
        players = (1..=playerCount).map(usize::from).collect();

        match outCount {
            0 => out.clear(),
            1 => {
                out.push(players.remove(uWrap(gameIndex, players.len())));
            }
            2 => {
                if uWrap(gameIndex, players.len()) <= iWrap(-(gameIndex as isize + 1), players.len()) {
                    out.push(players.remove(uWrap(gameIndex, players.len())));
                    out.push(players.remove(iWrap(-(gameIndex as isize + 1), players.len())));
                } else {
                    out.push(players.remove(uWrap(gameIndex, players.len())));
                    out.push(players.remove(iWrap(-(gameIndex as isize), players.len())));
                }
            }
            3 => {
                if uWrap(gameIndex + 1, players.len()) <= iWrap(-(gameIndex as isize + 1), players.len()) {
                    out.push(players.remove(uWrap(gameIndex, players.len())));
                    out.push(players.remove(uWrap(gameIndex, players.len())));
                    out.push(players.remove(iWrap(-(gameIndex as isize + 1), players.len())));
                } else {
                    out.push(players.remove(uWrap(gameIndex, players.len())));
                    out.push(players.remove(uWrap(gameIndex, players.len())));
                    out.push(players.remove(iWrap(-(gameIndex as isize), players.len())));
                }
            }
            _ => panic!("Something went wrong, outCount is not valid")
        }
    
        for tableIndex in 0..tableCount {

            let mut pAA = rand::thread_rng().gen_range(0..players.len());
            let mut pAB = rand::thread_rng().gen_range(0..players.len());

            let mut teamAChecks = 50;

            while pAA == pAB || (teamAChecks >= 0 && usedPartners.contains(&(players[pAA], players[pAB]))) {
                
                pAA = rand::thread_rng().gen_range(0..players.len());
                pAB = rand::thread_rng().gen_range(0..players.len());
                teamAChecks -= 1;
            }

            let teamA = (players[pAA], players[pAB]);

            usedPartners.push((teamA.0, teamA.1));
            usedPartners.push((teamA.1, teamA.0));

            if pAB > pAA {
                pAB -= 1;
            }
            players.remove(pAA);
            players.remove(pAB);

            let mut pBA = rand::thread_rng().gen_range(0..players.len());
            let mut pBB = rand::thread_rng().gen_range(0..players.len());

            let mut teamBChecks = 50;
            let mut neighborCheckThreshold = 20;

            while pBA == pBB || (neighborCheckThreshold >= 0 && (usedNeighbors.contains(&(teamA.0, players[pBA]))
                                                                || usedNeighbors.contains(&(players[pBA], teamA.1))
                                                                || usedNeighbors.contains(&(teamA.1, players[pBB]))
                                                                || usedNeighbors.contains(&(players[pBB], teamA.0)))) {

                while pBA == pBB || (teamBChecks >= 0 && usedPartners.contains(&(players[pBA], players[pBB]))) {
                    
                    pBA = rand::thread_rng().gen_range(0..players.len());
                    pBB = rand::thread_rng().gen_range(0..players.len());
                    teamBChecks -= 1;
                }

                neighborCheckThreshold -= 1;
            }

            let teamB = (players[pBA], players[pBB]);

            usedNeighbors.push((teamA.0, teamB.0));
            usedNeighbors.push((teamB.0, teamA.0));
            usedNeighbors.push((teamB.0, teamA.1));
            usedNeighbors.push((teamA.1, teamB.0));
            usedNeighbors.push((teamA.1, teamB.1));
            usedNeighbors.push((teamB.1, teamA.1));
            usedNeighbors.push((teamB.1, teamA.0));
            usedNeighbors.push((teamA.0, teamB.1));

            usedPartners.push((teamB.0, teamB.1));
            usedPartners.push((teamB.1, teamB.0));

            if pBB > pBA {
                pBB -= 1;
            }
            players.remove(pBA);
            players.remove(pBB);

            game.push((teamA, teamB));
        }

        full.insert(gameIndex, (game.clone(), out.clone()));
    }

    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();
    irs.shuffle(&mut full, &mut rng);

    full
}

fn formatPlayersTuple(playerTup: (usize, usize), separator: String, displayNames: bool, playerNames: Vec<String>) -> String{

    let mut output: String = String::new();

    if displayNames {
        let mut playerA = playerNames[playerTup.0 - 1].clone();
        let mut playerB = playerNames[playerTup.1 - 1].clone();

        if playerA.is_empty() {
            playerA = format!("Player {}", playerTup.0);
        }

        if playerB.is_empty() {
            playerB = format!("Player {}", playerTup.1);
        }

        output = format!("{}{}{}", playerA, separator, playerB);
    } else {
        output = format!("{}{}{}", playerTup.0, separator, playerTup.1);
    }

    output
}

fn formatPlayersVector(playerVec: Vec<usize>, separator: String, displayNames: bool, playerNames: Vec<String>) -> String{

    let mut output: String = String::new();

    for index in 0..playerVec.len() {
        
        if index != 0 {
            output += &separator;
        }

        if displayNames {
            let mut name = playerNames[playerVec[index] - 1].clone();

            if name.is_empty() {
                name = format!("Player {}", playerVec[index]);
            }

            output += &playerNames[playerVec[index] - 1];
        } else {
            output += &playerVec[index].to_string();
        }
    }

    output
}

fn generateSpace(spaces: i32) -> String {

    let mut str = String::new();

    for i in 0..spaces {
        str += " ";
    }

    str
}

fn iWrap(index: isize, size: usize) -> usize {

    let mut result = index;

    let iSize = size.try_into().unwrap();

    while result >= iSize {
        result -= iSize;
    }

    while result < 0 {
        result += iSize;
    }

    result.try_into().unwrap()
}

fn uWrap(index: usize, size: usize) -> usize {

    let mut result = index;

    while result >= size {
        result -= size;
    }

    result
}