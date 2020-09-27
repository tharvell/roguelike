use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{min, max};
use specs_derive::Component;


#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}


struct State {
    ecs: World
}
impl GameState for State{
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rogue Stuff!!")
        .build();

    let mut gs = State { 
        ecs: World::new()
      };
    
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs
        .create_entity()
        .with(Position {x: 40, y:25 })
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
    
    for i in 0..10 {
        gs.ecs
          .create_entity()
          .with(Position{x: i * 7, y: 20})
          .with(Renderable{
              glyph: rltk::to_cp437('@'),
              fg: RGB::named(rltk::RED),
              bg: RGB::named(rltk::BLACK),
          })
          .build();
    }
        
    rltk::main_loop(context, gs)
}
