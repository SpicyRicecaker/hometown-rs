use hometown::rain::RainDropMachineWrapper;
use hometown::game::Game;

fn main() {
    let mut game = Game::new();
    (0..2).into_iter().for_each(|_| game.gen_raindrop());
    game.raindrops.iter().for_each(|f| {
        if let RainDropMachineWrapper::Falling(e) = f {
            dbg!(e.state.x, e.state.y);
        }
    });

    
    let (ctx, cb) = thomas::ContextBuilder::new()
        .with_world_dimension((game.x, game.y, game.z).into())
        .build();

    thomas::main::run(ctx, cb, game);
}
