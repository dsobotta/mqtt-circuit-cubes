use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind};
use crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};

use crate::app::App;
use crate::ui;

const AXIS_X: (KeyCode, KeyCode) = (KeyCode::Char('a'), KeyCode::Char('d'));
const AXIS_Y: (KeyCode, KeyCode) = (KeyCode::Char('w'), KeyCode::Char('s'));

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("Crossterm Demo", enhanced_graphics);
    let app_result = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        ),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn init_states(codes: &[KeyCode], key_map: &mut HashMap<KeyCode, KeyEventKind>) {
    for code in codes.iter() {
        key_map.insert(*code, KeyEventKind::Release);
    }
}

fn get_key_axis_1d(code_a: KeyCode, code_b: KeyCode, key_map: &HashMap<KeyCode, KeyEventKind>) -> i16 {
    let mut ret: i16 = 0;
    let kind_a = key_map.get(&code_a).expect("could not find state for keycode");
    let kind_b = key_map.get(&code_b).expect("could not find state for keycode");

    if let KeyEventKind::Press = kind_a {
        ret -= 1;
    }

    if let KeyEventKind::Press = kind_b {
        ret += 1;
    }

    ret
}

fn map_power_from_axis_2d(axis_x: i16, axis_y: i16, power_map: &[[(i16, i16); 3]; 3]) -> (i16, i16) {
    let x: usize = (axis_x + 1).try_into().expect("index out of range");
    let y: usize = (axis_y + 1).try_into().expect("index out of range");
    power_map[y][x]
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, tick_rate: Duration) -> Result<(), Box<dyn Error>>
// where
    // B::Error: 'static,
{
    let mut last_tick = Instant::now();

    let input_keys = [
        //axis 1
        KeyCode::Char('w'),
        KeyCode::Char('s'),
        //axis 2
        KeyCode::Char('a'),
        KeyCode::Char('d'),
        //axis 3
        KeyCode::Char('q'),
        KeyCode::Char('e'),
    ];

    let full_speed: i16 = 255;
    let turn_speed: i16 = 160;
    let spin_speed: i16 = 128;

    let tank_power_map = [
        [
            (turn_speed, full_speed),
            (full_speed, full_speed),
            (full_speed, turn_speed),
        ],
        [(-spin_speed, spin_speed), (0, 0), (spin_speed, -spin_speed)],
        [
            (-turn_speed, -full_speed),
            (-full_speed, -full_speed),
            (-full_speed, -turn_speed),
        ],
    ];

    let mut key_states: HashMap<KeyCode, KeyEventKind> = HashMap::new();
    init_states(&input_keys, &mut key_states);

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            app.on_tick();
            last_tick = Instant::now();
            continue;
        }

        let event = event::read().expect("failed to read event");

        if let Some(key) = event.as_key_event() {
            let code = key.code;
            let kind = key.kind;
            key_states.insert(code, kind);
            let kind_str = match key.kind {
                KeyEventKind::Press => "Pressed",
                KeyEventKind::Release => "Released",
                KeyEventKind::Repeat => "Repeat",
            };
            app.append_log(format!("KeyEvent: {code}, {kind_str}"));
        }

        let axis_x = get_key_axis_1d(AXIS_X.0, AXIS_X.1, &key_states);
        let axis_y = get_key_axis_1d(AXIS_Y.0, AXIS_Y.1, &key_states);
        // let (pow_a, pow_b) = map_power_from_axis_2d(axis_x, axis_y, &tank_power_map);
        app.append_log(format!("Axis_X+Y: {axis_x}, {axis_y}"));
        // app.append_log(format!("Power_Tank: {pow_a}, {pow_b}"));

        if let Some(key) = event.as_key_release_event() {
            match key.code {
                KeyCode::Char('1') => app.set_tab(0),
                KeyCode::Char('2') => app.set_tab(1),
                KeyCode::Char('3') => app.set_tab(2),
                KeyCode::Char('4') => app.set_tab(3),
                KeyCode::Char('w') | KeyCode::Up => app.on_up(),
                KeyCode::Char('a') | KeyCode::Left => app.on_left(),
                KeyCode::Char('s') | KeyCode::Down => app.on_down(),
                KeyCode::Char('d') | KeyCode::Right => app.on_right(),
                KeyCode::Char(c) => app.on_key(c),
                _ => {}
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
