use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyEventKind, poll};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};

use crate::app::{App, CircuitCubeCommand, CircuitCubeTerminal};
use crate::ui;

use gilrs::{self, EventType, Gilrs};

const MAX_DELAY_BETWEEN_FRAMES: u32 = 1_000_000_000u32 / 60;
const TANK_FULL_SPEED: i16 = 255;
const TANK_TURN_SPEED: i16 = 160;
const TANK_SPIN_SPEED: i16 = 128;
const TANK_POWER_MAP: [[(i16, i16); 3]; 3] = [
    [(TANK_TURN_SPEED, TANK_FULL_SPEED), (TANK_FULL_SPEED, TANK_FULL_SPEED), (TANK_FULL_SPEED, TANK_TURN_SPEED)],
    [(-TANK_SPIN_SPEED, TANK_SPIN_SPEED), (0, 0), (TANK_SPIN_SPEED, -TANK_SPIN_SPEED)],
    [(-TANK_TURN_SPEED, -TANK_FULL_SPEED), (-TANK_FULL_SPEED, -TANK_FULL_SPEED), (-TANK_FULL_SPEED, -TANK_TURN_SPEED)],
];

const BODY_BANK: usize = 0;
const BODY_TERMINAL: CircuitCubeTerminal = CircuitCubeTerminal::B;
const BODY_TILT_SPEED: f32 = 100.0;
const BODY_UP_BUTTON: gilrs::Button = gilrs::Button::RightTrigger;
const BODY_DOWN_BUTTON: gilrs::Button = gilrs::Button::East;

const HEAD_BANK: usize = 1;
const HEAD_TERMINAL: CircuitCubeTerminal = CircuitCubeTerminal::A;
const HEAD_TURN_SPEED: f32 = 100.0;
const HEAD_LEFT_BUTTON: gilrs::Button = gilrs::Button::West;
const HEAD_RIGHT_BUTTON: gilrs::Button = gilrs::Button::South;

const CLAW_BANK: usize = 1;
const CLAW_TERMINAL: CircuitCubeTerminal = CircuitCubeTerminal::C;
const CLAW_SPEED: f32 = 100.0;
const OPEN_CLAW_BUTTON: gilrs::Button = gilrs::Button::RightTrigger2;
const CLOSE_CLAW_BUTTON: gilrs::Button = gilrs::Button::North;

const RECORD_BUTTON: gilrs::Button = gilrs::Button::Select;
const PLAY_BUTTON: gilrs::Button = gilrs::Button::Start;

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

fn handle_button_change(button: gilrs::Button, value: f32, app: &mut App) {
    match button {
        HEAD_LEFT_BUTTON => {
            app.circuit_cube_cmd(CircuitCubeCommand::SetPower(HEAD_BANK, HEAD_TERMINAL, HEAD_TURN_SPEED * -value));
            app.log_debug("HEAD_LEFT".to_string());
        }
        HEAD_RIGHT_BUTTON => {
            app.circuit_cube_cmd(CircuitCubeCommand::SetPower(HEAD_BANK, HEAD_TERMINAL, HEAD_TURN_SPEED * value));
            app.log_debug("HEAD_RIGHT".to_string());
        }

        BODY_UP_BUTTON => {
            app.circuit_cube_cmd(CircuitCubeCommand::SetPower(BODY_BANK, BODY_TERMINAL, BODY_TILT_SPEED * -value));
            app.log_debug("BODY_UP".to_string());
        }
        BODY_DOWN_BUTTON => {
            app.circuit_cube_cmd(CircuitCubeCommand::SetPower(BODY_BANK, BODY_TERMINAL, BODY_TILT_SPEED * value));
            app.log_debug("BODY_DOWN".to_string());
        }

        OPEN_CLAW_BUTTON => {
            app.circuit_cube_cmd(CircuitCubeCommand::SetPower(CLAW_BANK, CLAW_TERMINAL, CLAW_SPEED * -value));
            app.log_debug("OPEN_CLAW".to_string());
        }
        CLOSE_CLAW_BUTTON => {
            app.circuit_cube_cmd(CircuitCubeCommand::SetPower(CLAW_BANK, CLAW_TERMINAL, CLAW_SPEED * value));
            app.log_debug("CLOSE_CLAW".to_string());
        }

        RECORD_BUTTON => {
            if value > 0.0 {
                app.log_debug("RECORDING...".to_string());
            }
        }
        PLAY_BUTTON => {
            if value > 0.0 {
                app.log_debug("PLAYBACK...".to_string());
            }
        }
        _ => {
            app.log_debug(format!("unhandled button: {:?}", button));
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    // for (_id, gamepad) in gilrs.gamepads() {
    //     println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    // }

    let mut active_gamepad = None;

    'running: loop {
        // Examine new events
        while let Some(gilrs::Event { id, event, time, .. }) = gilrs.next_event() {
            active_gamepad = Some(id);
            match event {
                EventType::ButtonPressed(..) => (),
                EventType::ButtonReleased(..) => (),
                EventType::ButtonChanged(button, value, ..) => handle_button_change(button, value, &mut app),
                // EventType::AxisChanged(axis, value, ..) => {

                // }
                _ => {
                    app.log_debug(format!("{:?} New event from {}: {:?}", time, id, event));
                }
            }
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(gilrs::Button::South) {
                // println!("Button South is pressed (XBox - A, PS - X)");
            }
        }

        if poll(Duration::new(0, MAX_DELAY_BETWEEN_FRAMES)).expect("failed to poll for events") {
            let event = event::read().expect("failed to read event");

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
                    event::KeyCode::Char('q') => break 'running,
                    event::KeyCode::Char(c) => app.on_key(c),
                    _ => {}
                }
            }
        }

        terminal.draw(|frame| ui::render(frame, &mut app))?;
    }

    Ok(())
}
