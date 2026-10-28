//! Discord's Rich Presence: while the game runs, Discord shows where the
//! player is ("Dryfield – Motel, Room 6"), their EXP and BP, and for how
//! long they have played. [general] discord turns it off; without Discord
//! running nothing happens. The game's fork reports the state
//! (pad_input.c `pc_presence_state`); a thread of this module polls it and
//! talks to Discord through its local pipe.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use discord_rich_presence::activity::{Activity, Assets, Timestamps};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

/// The PE2PC application on Discord's developer portal (its name is the
/// "Playing" line, its art assets the pictures).
const APPLICATION_ID: &str = "1565134006784364666";

/// How often the state is looked at, and Discord tried again when absent.
const POLL: Duration = Duration::from_secs(15);

unsafe extern "C" {
    fn pc_presence_state(stage: *mut i32, area: *mut i32, exp: *mut i32, bp: *mut i32);
}

/// Starts the presence, unless turned off or in an unattended run.
pub fn start() {
    let unattended = ["PE2_INPUT", "PE2_REPLAY"].iter().any(|v| std::env::var_os(v).is_some());
    if unattended || !crate::config::get().general.discord.unwrap_or(true) {
        return;
    }
    let pt = crate::config::get().general.language.as_deref() == Some("pt");
    let since = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64);
    let _ = std::thread::Builder::new().name("discord".into()).spawn(move || run(pt, since));
}

fn run(pt: bool, since: i64) {
    let mut client = DiscordIpcClient::new(APPLICATION_ID);
    let mut connected = false;
    let mut shown = None;
    loop {
        if !connected {
            connected = client.connect().is_ok();
            if connected {
                eprintln!("pe2: discord: showing the game");
            }
            shown = None;
        }
        if connected {
            let (mut stage, mut area, mut exp, mut bp) = (0, 0, 0, 0);
            // SAFETY: reads four of the game's globals into these locals; a
            // read racing the game thread only shows stale numbers.
            unsafe { pc_presence_state(&mut stage, &mut area, &mut exp, &mut bp) };
            let lines = presence(pt, stage, area, exp, bp);
            if shown.as_ref() != Some(&lines) {
                let (details, state) = &lines;
                let mut activity = Activity::new()
                    .details(details.as_str())
                    .timestamps(Timestamps::new().start(since))
                    .assets(Assets::new().large_image("logo").large_text("PE2PC"));
                if let Some(state) = state {
                    activity = activity.state(state.as_str());
                }
                if client.set_activity(activity).is_ok() {
                    shown = Some(lines);
                } else {
                    let _ = client.close();
                    connected = false;
                }
            }
        }
        std::thread::sleep(POLL);
    }
}

/// The two lines Discord shows: where the player is, and their EXP and BP
/// (none before a game has started).
fn presence(pt: bool, stage: i32, area: i32, exp: i32, bp: i32) -> (String, Option<String>) {
    if stage == 0 {
        let title = if pt { "Na tela de título" } else { "At the title screen" };
        return (title.to_string(), None);
    }
    let details = match place(pt, stage, area) {
        (place, Some(room)) => format!("{place} – {room}"),
        (place, None) => place.to_string(),
    };
    (details, Some(format!("EXP {exp} · BP {bp}")))
}

/// The place (a town, a building) and the room of `area` in `stage`
/// (main/areas.h), in English or Portuguese. Rooms with no name of their
/// own (never visited in play, or unknown) show only the place.
fn place(pt: bool, stage: i32, area: i32) -> (&'static str, Option<&'static str>) {
    let (place_en, place_pt) = match (stage, area) {
        (1, 18..=21) => ("M.I.S.T.", "M.I.S.T."),
        (1, _) => ("Akropolis Tower", "Torre Akropolis"),
        (2, _) => ("Dryfield", "Dryfield"),
        (3, _) => ("Dryfield (Night)", "Dryfield (noite)"),
        (4, 1..=8) => ("Mine", "Mina"),
        (4, _) | (5, 1..=6 | 22..=25 | 28) => ("Shelter", "Abrigo"),
        (5, _) => ("Neo Ark", "Neo Ark"),
        _ => ("Parasite Eve II", "Parasite Eve II"),
    };
    let day = if stage == 3 { 2 } else { stage };
    let room = ROOMS.iter().find(|r| r.0 == day && r.1 == area).map(|r| if pt { r.3 } else { r.2 });
    (if pt { place_pt } else { place_en }, room)
}

/// (stage, area, English, Portuguese): each room's name. Dryfield by night
/// (stage 3) has the day's rooms (stage 2).
const ROOMS: &[(i32, i32, &str, &str)] = &[
    (1, 1, "Square", "Praça"),
    (1, 2, "East Elevator Hall", "Saguão do elevador leste"),
    (1, 3, "Patio", "Pátio"),
    (1, 4, "Cafeteria", "Cafeteria"),
    (1, 5, "Plaza", "Plaza"),
    (1, 6, "Security Room", "Sala de segurança"),
    (1, 7, "Hallway", "Corredor"),
    (1, 8, "Fountain", "Fonte"),
    (1, 9, "Forked Road", "Bifurcação"),
    (1, 10, "Observatory", "Observatório"),
    (1, 11, "Promenade", "Passeio"),
    (1, 12, "Sanctuary", "Santuário"),
    (1, 13, "Roof Garden", "Jardim do terraço"),
    (1, 14, "Bridge", "Ponte"),
    (1, 15, "Fire Escape", "Saída de incêndio"),
    (1, 16, "Helicopter Landing Pad", "Heliponto"),
    (1, 17, "West Elevator Hall", "Saguão do elevador oeste"),
    (1, 19, "Parking", "Estacionamento"),
    (1, 20, "Shooting Range", "Estande de tiro"),
    (2, 1, "Gas Station", "Posto de gasolina"),
    (2, 2, "Main Street", "Rua principal"),
    (2, 3, "General Store", "Armazém"),
    (2, 5, "Back Street", "Rua dos fundos"),
    (2, 6, "Souvenir Shop", "Loja de lembranças"),
    (2, 7, "Warehouse", "Depósito"),
    (2, 9, "Dilapidated House", "Casa abandonada"),
    (2, 11, "Motel, Room 1", "Motel, Quarto 1"),
    (2, 12, "Motel, Room 2", "Motel, Quarto 2"),
    (2, 13, "Motel, Room 3", "Motel, Quarto 3"),
    (2, 14, "Motel, Room 4", "Motel, Quarto 4"),
    (2, 15, "Parking Lot", "Estacionamento"),
    (2, 16, "Toilet", "Banheiro"),
    (2, 17, "Motel Lobby", "Recepção do motel"),
    (2, 18, "Saloon G&R", "Saloon G&R"),
    (2, 19, "G&R Kitchen", "Cozinha do G&R"),
    (2, 20, "Water Tower", "Caixa d'água"),
    (2, 21, "Water Tank", "Reservatório"),
    (2, 22, "Breezeway", "Passagem coberta"),
    (2, 23, "Factory", "Fábrica"),
    (2, 24, "Garage", "Garagem"),
    (2, 25, "Driveway", "Entrada da garagem"),
    (2, 26, "Junk Yard", "Ferro-velho"),
    (2, 27, "Trailer Coach", "Trailer"),
    (2, 28, "Motel, Room 5", "Motel, Quarto 5"),
    (2, 29, "Motel Balcony", "Sacada do motel"),
    (2, 30, "Motel, Room 6", "Motel, Quarto 6"),
    (2, 31, "Motel Loft", "Sótão do motel"),
    (2, 32, "Water Hole", "Poço"),
    (2, 34, "Cellar", "Porão"),
    (2, 38, "Underpass", "Passagem subterrânea"),
    (4, 1, "Mesa", "Mesa"),
    (4, 2, "Cavern", "Caverna"),
    (4, 3, "Tunnel Entrance", "Entrada do túnel"),
    (4, 4, "Tunnel", "Túnel"),
    (4, 5, "Gorge", "Desfiladeiro"),
    (4, 6, "Refuge", "Refúgio"),
    (4, 7, "Forked Tunnel", "Túnel bifurcado"),
    (4, 8, "Secret Passage", "Passagem secreta"),
    (4, 9, "B1 Elevator Hall", "B1, Saguão do elevador"),
    (4, 10, "B1 South Maintenance Walkway", "B1, Passarela de manutenção sul"),
    (4, 11, "B1 Storeroom", "B1, Almoxarifado"),
    (4, 12, "B1 North Maintenance Walkway", "B1, Passarela de manutenção norte"),
    (4, 13, "B1 Armory", "B1, Arsenal"),
    (4, 14, "B1 Sleeping Quarters", "B1, Dormitório"),
    (4, 15, "B1 Main Corridor", "B1, Corredor principal"),
    (4, 16, "B1 Sterilization Room", "B1, Sala de esterilização"),
    (4, 17, "B1 Pod Access Tunnel", "B1, Túnel de acesso ao pod"),
    (4, 18, "B1 Control Room", "B1, Sala de controle"),
    (4, 19, "B1 Access Tunnel", "B1, Túnel de acesso"),
    (4, 20, "B1 Underground Parking", "B1, Garagem subterrânea"),
    (4, 21, "B1 Golem Freezer", "B1, Câmara fria dos Golems"),
    (4, 22, "B2 Pod Bottom", "B2, Fundo do pod"),
    (4, 23, "B1 Pod Service Gantry", "B1, Passarela de serviço do pod"),
    (4, 24, "B1 Transfer Tunnel", "B1, Túnel de transferência"),
    (4, 25, "B1 Control Room Access Tunnel", "B1, Túnel de acesso à sala de controle"),
    (4, 26, "B2 Elevator", "B2, Elevador"),
    (4, 27, "B2 Elevator Hall", "B2, Saguão do elevador"),
    (4, 28, "B2 South Maintenance Walkway", "B2, Passarela de manutenção sul"),
    (4, 29, "B2 Operating Room", "B2, Sala de cirurgia"),
    (4, 30, "B2 North Maintenance Walkway", "B2, Passarela de manutenção norte"),
    (4, 31, "B2 Laboratory", "B2, Laboratório"),
    (4, 32, "B2 Breeding Room", "B2, Sala de criação"),
    (4, 33, "B2 Main Corridor", "B2, Corredor principal"),
    (4, 34, "B2 Septic Tank", "B2, Fossa séptica"),
    (4, 35, "B2 Pod Access Tunnel", "B2, Túnel de acesso ao pod"),
    (4, 38, "1F Heliport", "1F, Heliponto"),
    (4, 39, "B3 Dumping Hole", "B3, Fosso de descarte"),
    (4, 40, "B3 Garbage Incinerator", "B3, Incinerador de lixo"),
    (4, 41, "B3 Incinerator Control Room", "B3, Sala de controle do incinerador"),
    (4, 42, "B3 Elevator Hall", "B3, Saguão do elevador"),
    (4, 43, "B4 Lower Sewer", "B4, Esgoto inferior"),
    (4, 44, "B4 Upper Sewer", "B4, Esgoto superior"),
    (4, 45, "B4 Reservoir", "B4, Reservatório"),
    (4, 46, "B4 Water Supply", "B4, Abastecimento de água"),
    (5, 1, "1F Parking Garage", "1F, Estacionamento"),
    (5, 2, "1F Vehicular Airlock", "1F, Eclusa de veículos"),
    (5, 3, "1F Bulwark", "1F, Muralha"),
    (5, 4, "1F Heliport", "1F, Heliponto"),
    (5, 5, "1F Airlock", "1F, Eclusa"),
    (5, 6, "1F Guardroom", "1F, Sala da guarda"),
    (5, 7, "Observatory", "Observatório"),
    (5, 8, "Eve Access Tunnel", "Túnel de acesso da Eve"),
    (5, 9, "Eve Elevator", "Elevador da Eve"),
    (5, 10, "North Promenade", "Passeio norte"),
    (5, 11, "Forest Zone", "Zona da floresta"),
    (5, 12, "Submarine Tunnel", "Túnel do submarino"),
    (5, 13, "Pavilion", "Pavilhão"),
    (5, 14, "Island", "Ilha"),
    (5, 15, "Garden", "Jardim"),
    (5, 16, "Power Plant 2", "Usina 2"),
    (5, 17, "Power Plant 1", "Usina 1"),
    (5, 18, "Savanna Zone", "Zona da savana"),
    (5, 19, "South Promenade", "Passeio sul"),
    (5, 20, "Altar", "Altar"),
    (5, 21, "Shrine", "Santuário"),
    (5, 22, "B6 Nursery", "B6, Berçário"),
    (5, 23, "B6 Growth Room", "B6, Sala de crescimento"),
    (5, 24, "B6 Corridor", "B6, Corredor"),
    (5, 25, "B6 Training Room", "B6, Sala de treinamento"),
    (5, 27, "Bridge", "Ponte"),
    (5, 28, "1F Tent", "1F, Tenda"),
    (5, 29, "Woodland Path", "Trilha da mata"),
    (5, 30, "Submarine Gallery", "Galeria do submarino"),
    (5, 32, "Pyramid", "Pirâmide"),
    (5, 33, "Substation", "Subestação"),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// A room shows with its place, by night as by day; one with no name
    /// shows the place alone; the title screen has no EXP or BP.
    #[test]
    fn presence_lines() {
        assert_eq!(
            presence(true, 2, 30, 1200, 3000),
            ("Dryfield – Motel, Quarto 6".into(), Some("EXP 1200 · BP 3000".into()))
        );
        assert_eq!(presence(false, 3, 30, 0, 0).0, "Dryfield (Night) – Motel, Room 6");
        assert_eq!(presence(false, 4, 36, 0, 0).0, "Shelter");
        assert_eq!(presence(false, 0, 0, 5, 5), ("At the title screen".into(), None));
    }
}
