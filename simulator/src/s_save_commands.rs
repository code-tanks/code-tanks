use bevy::{prelude::*, utils::HashSet};
use serde_json::{json, to_value};
use std::{fs::OpenOptions, io::Write};

use crate::{
    c_command::{CCommands, CommandSource},
    c_health::Health,
    c_tank::*,
    TankIds, TickState,
};
use bevy::app::AppExit;

pub fn save_commands(
    mut state: ResMut<TickState>,
    tank_ids_state: Res<TankIds>,
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
            CCommands::NONE
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
    state.tick = state.tick + 1;

    let early_stop = dead_count >= tanks.len() - 1;
    
    if state.tick > TickState::MAXIMUM_SIMULATION_TICKS || early_stop {
        println!("early_stop: {}", early_stop);
        // TODO save results of the simulation (winner, damage given, damage taken, time alive)
        let mut j = json!({});
        let all_tank_ids: &mut Vec<&str> = &mut Vec::new();
        let mut best_idx: usize = 0;
        let mut dup: bool = false;
        for (i, tank_id) in tank_ids_state.tank_ids.iter().enumerate() {
            let ti = &tank_id[tank_id.find("-").unwrap() + 1..];
            let dmg = damages_dealt[i].damage_dealt;

            j[ti] = json!({
                "tank_id": ti,
                "index": i,
                "health": healths[i].val,
                "damage_given": dmg,
            });

            let t = &ti[..ti.find("-").unwrap()];

            all_tank_ids.push(&t);

            if dmg == damages_dealt[best_idx].damage_dealt && i != 0 {
                dup = true;
            }
            if dmg > damages_dealt[best_idx].damage_dealt {
                dup = false;
                best_idx = i;
            }
        }
        j["tanks"] = to_value(HashSet::from_iter(all_tank_ids.clone())).unwrap();
        j["winner"] = if dup {
            "".into()
        } else {
            all_tank_ids[best_idx].into()
        };
        j["winner_index"] = if dup {
            (-1i32).into()
        } else {
            best_idx.into()
        };        
        println!("{}", j);

        f.write_all(j.to_string().as_bytes())
            .expect("Unable to write data");
        exit.send(AppExit);
    }
}
