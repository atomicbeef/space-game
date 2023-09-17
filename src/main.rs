use bevy::prelude::*;

use space_game::app_setup::{SetupGame, SetupBevyPlugins, SetupDebug};
use space_game::fixed_update::{SetupFixedTimeStepSchedule, SetupRapier};

fn main() {
    App::new()
        .setup_bevy_plugins()
        .setup_fixed_timestep_schedule()
        .setup_rapier()
        .setup_game()
        .setup_debug()
        .run();
}
