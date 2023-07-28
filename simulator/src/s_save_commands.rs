use bevy::{prelude::*, utils::HashSet};
use ct_api::Commands;
use serde_json::{json, to_value};
use std::{fs::OpenOptions, io::Write};

use crate::{c_command_source::CommandSource, c_health::Health, c_tank::*, TickState};
use bevy::app::AppExit;

pub fn save_commands(
    mut state: ResMut<TickState>,
    tank_state: Res<AllTankInfo>,
    mut exit: EventWriter<AppExit>,
    query: Query<&CommandSource>,
    tanks: Query<(&Transform, &Tank)>,
    radars: Query<&Transform, With<Radar>>,
    guns: Query<&Transform, With<Gun>>,
    healths: Query<&Health, With<Tank>>,
    damage_dealt: Query<&DamageDealer, With<Tank>>,
) {
    let tanks: Vec<(&Transform, &Tank)> = tanks.iter().collect();
    let radars: Vec<&Transform> = radars.iter().collect();
    let guns: Vec<&Transform> = guns.iter().collect();
    let healths: Vec<&Health> = healths.iter().collect();
    let damages_dealt: Vec<&DamageDealer> = damage_dealt.iter().collect();

    let mut f = OpenOptions::new()
        .append(true)
        .open("./sim.txt")
        .expect("Unable to open file");

    let mut dead_count = 0usize;

    for (i, command_receiver) in query.iter().enumerate() {
        let grouped_commands = if command_receiver.queue.is_empty() {
            Commands::NONE
        } else {
            command_receiver.queue[0]
        };

        // println!("save_commands {:?}", grouped_commands);

        f.write_all(
            format!(
                "{}|{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                grouped_commands,
                tanks[i].0.translation.x,
                tanks[i].0.translation.y,
                tanks[i].0.rotation.x, // unused
                tanks[i].0.rotation.y, // unused
                tanks[i].0.rotation.z,
                tanks[i].0.rotation.w,
                radars[i].rotation.x, // unused
                radars[i].rotation.y, // unused
                radars[i].rotation.z,
                radars[i].rotation.w,
                guns[i].rotation.x, // unused
                guns[i].rotation.y, // unused
                guns[i].rotation.z,
                guns[i].rotation.w,
            )
            .to_string()
            .as_bytes(),
        )
        .expect("Unable to write data");

        if healths[i].val <= 0 {
            dead_count += 1;
        }

        // println!("commands remaining {:?}", command_receiver.queue);
    }

    let early_stop = dead_count >= tanks.len() - 1;

    if state.count >= TickState::MAXIMUM_SIMULATION_TICKS || early_stop {
        state.count = TickState::MAXIMUM_SIMULATION_TICKS;
        println!("early_stop: {}", early_stop);
        // TODO save results of the simulation (winner, damage given, damage taken, time alive)
        let mut j = json!({});
        let mut best_idx: usize = 0;
        let mut dup: bool = false;
        for tank_info in &tank_state.all
        {
            let dmg = damages_dealt[tank_info.index].damage_dealt;

            j[tank_info.id.to_string()] = json!({
                "tank_hash": tank_info.hash,
                "index": tank_info.index,
                "health": healths[tank_info.index].val,
                "damage_given": dmg,
            });

            if dmg == damages_dealt[best_idx].damage_dealt && tank_info.index != 0 {
                dup = true;
            }
            if dmg > damages_dealt[best_idx].damage_dealt {
                dup = false;
                best_idx = tank_info.index;
            }
        }
        j["tanks"] = to_value(HashSet::from_iter(tank_state.all.iter().map(|f| f.hash.to_string()))).unwrap();
        j["winner"] = if dup {
            "".into()
        } else {
            tank_state.all[best_idx].id.to_string().into()
        };
        j["winner_index"] = if dup { (-1i32).into() } else { best_idx.into() };
        println!("{}", j);

        f.write_all(j.to_string().as_bytes())
            .expect("Unable to write data");
        exit.send(AppExit);
    }
}
