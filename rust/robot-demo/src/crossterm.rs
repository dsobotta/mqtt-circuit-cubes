use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

use crate::app::App;
use crate::ui;

// const AXIS_X: (keyboard::KeyCode, keyboard::KeyCode) = (KeyCode::Char('a'), KeyCode::Char('d'));
// const AXIS_Y: (KeyCode, KeyCode) = (KeyCode::Char('w'), KeyCode::Char('s'));

pub fn run(enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    //enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("Crossterm Demo", enhanced_graphics);
    let app_result = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

// fn init_states(codes: &[KeyCode], key_map: &mut HashMap<KeyCode, bool>) {
//     for code in codes.iter() {
//         key_map.insert(*code, false);
//     }
// }

// fn get_key_axis_1d(code_a: KeyCode, code_b: KeyCode, key_map: &HashMap<KeyCode, bool>) -> i16 {
//     let mut ret: i16 = 0;
//     let kind_a = key_map.get(&code_a).expect("could not find state for keycode");
//     let kind_b = key_map.get(&code_b).expect("could not find state for keycode");

//     if *kind_a {
//         ret -= 1;
//     }

//     if *kind_b {
//         ret += 1;
//     }

//     ret
// }

// fn map_power_from_axis_2d(axis_x: i16, axis_y: i16, power_map: &[[(i16, i16); 3]; 3]) -> (i16, i16) {
//     let x: usize = (axis_x + 1).try_into().expect("index out of range");
//     let y: usize = (axis_y + 1).try_into().expect("index out of range");
//     power_map[y][x]
// }

// pub fn process_key(&mut self, key: &sdl::event::Event) {
//     if let sdl::event::Event::Key(k, pressed, _, _) = key {
//         // Key(_, false, _) means key up, true key down
//         self.pressed_keys.insert(*k as isize, *pressed).map_or_else(|| (), |_| ())
//     }
// }

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>>
// where
    // B::Error: 'static,
{
    // let input_keys = [
    //     //axis 1
    //     KeyCode::Char('w'),
    //     KeyCode::Char('s'),
    //     //axis 2
    //     KeyCode::Char('a'),
    //     KeyCode::Char('d'),
    //     //axis 3
    //     KeyCode::Char('q'),
    //     KeyCode::Char('e'),
    // ];

    let full_speed: i16 = 255;
    let turn_speed: i16 = 160;
    let spin_speed: i16 = 128;

    let tank_power_map = [
        [(turn_speed, full_speed), (full_speed, full_speed), (full_speed, turn_speed)],
        [(-spin_speed, spin_speed), (0, 0), (spin_speed, -spin_speed)],
        [(-turn_speed, -full_speed), (-full_speed, -full_speed), (-full_speed, -turn_speed)],
    ];

    // let mut key_states: HashMap<keyboard::Keycode, bool> = HashMap::new();
    // init_states(&input_keys, &mut key_states);

    'running: loop {
        // if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Q) {
        //     break 'running;
        // }

        // for event in event_pump.poll_iter() {
        //     app.append_log(format!("SDL event: {:?}", event));
        //     match event {
        //         Event::Quit { .. } | Event::KeyDown { keycode: Some(keyboard::Keycode::Q), .. } => {
        //             app.append_log("exiting".to_string());
        //             break 'running;
        //         }
        //         e => {
        //             app.append_log(format!("SDL event: {:?}", e));
        //         }
        //     }
        // }

        let event = event::read().expect("failed to read event");

        if let Some(key) = event.as_key_event() {
            let code = key.code;
            let pressed = key.kind == KeyEventKind::Press;
            // key_states.insert(code, pressed);
            let kind_str = match key.kind {
                KeyEventKind::Press => "Pressed",
                KeyEventKind::Release => "Released",
                KeyEventKind::Repeat => "Repeat",
            };
            app.append_log(format!("KeyEvent: {code}, {kind_str}"));
        }

        // let axis_x = get_key_axis_1d(AXIS_X.0, AXIS_X.1, &key_states);
        // let axis_y = get_key_axis_1d(AXIS_Y.0, AXIS_Y.1, &key_states);
        // let (pow_a, pow_b) = map_power_from_axis_2d(axis_x, axis_y, &tank_power_map);
        // app.append_log(format!("Axis_X+Y: {axis_x}, {axis_y}"));
        // app.append_log(format!("Power_Tank: {pow_a}, {pow_b}"));

        if let Some(key) = event.as_key_press_event() {
            match key.code {
                event::KeyCode::Char('1') => app.set_tab(0),
                event::KeyCode::Char('2') => app.set_tab(1),
                event::KeyCode::Char('3') => app.set_tab(2),
                event::KeyCode::Char('4') => app.set_tab(3),
                event::KeyCode::Char('w') | event::KeyCode::Up => app.on_up(),
                event::KeyCode::Char('a') | event::KeyCode::Left => app.on_left(),
                event::KeyCode::Char('s') | event::KeyCode::Down => app.on_down(),
                event::KeyCode::Char('d') | event::KeyCode::Right => app.on_right(),
                event::KeyCode::Char(c) => app.on_key(c),
                _ => {}
            }
        }

        terminal.draw(|frame| ui::render(frame, &mut app))?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
