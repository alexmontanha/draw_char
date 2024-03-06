extern crate ggez;
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Image};
use std::env;
use std::path::{Path, PathBuf};
use ggez::filesystem;

struct MainState {
    sprite_sheet: Image,
    position: (f32, f32),
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let resources_dir = env!("CARGO_MANIFEST_DIR").to_owned() + "\\resources";
        filesystem::mount(ctx, Path::new(&resources_dir), true);
        print!("Resources dir: {:?}", resources_dir);
        //let sprite_sheet_path = PathBuf::from(&resources_dir).join("bardo.png");
        let sprite_sheet_path = crate::PathBuf::from(&resources_dir).join("bardo.png");
        let sprite_sheet = Image::new(ctx, sprite_sheet_path)?;
        let position = (0.0, 0.0);
        Ok(MainState { sprite_sheet, position })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Atualize a posição do personagem aqui com base na entrada do teclado
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        // Desenhe seu personagem aqui usando self.sprite_sheet e self.position
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {
                self.position.1 -= 10.0;
            }
            KeyCode::Down => {
                self.position.1 += 10.0;
            }
            KeyCode::Left => {
                self.position.0 -= 10.0;
            }
            KeyCode::Right => {
                self.position.0 += 10.0;
            }
            _ => (), // Para todas as outras teclas, não faça nada
        }
    }
}

pub fn main() -> GameResult {
    match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(val) => println!("CARGO_MANIFEST_DIR: {}", val),
        Err(_e) => println!("Couldn't read CARGO_MANIFEST_DIR"),
    }
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
