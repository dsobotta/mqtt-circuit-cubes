use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyEventKind, poll};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::backend::{Backend, CrosstermBackend};

use circuitcubes::{CircuitCube, Command, Terminal, get_all_cubes};

use crate::app::App;
use crate::ui;

use gilrs::{self, EventType, Gilrs};

const MAX_DELAY_BETWEEN_FRAMES: u32 = 1_000_000_000u32 / 60;
const TANK_FULL_SPEED: i16 = 255;
const TANK_TURN_SPEED: i16 = 160;
const TANK_SPIN_SPEED: i16 = 128;
const TANK_POWER_MAP: [[(i16, i16); 3]; 3] = [
    [(TANK_TURN_SPEED, TANK_FULL_SPEED), (TANK_FULL_SPEED, TANK_FULL_SPEED), (TANK_FULL_SPEED, TANK_TURN_SPEED)],
    [(TANK_SPIN_SPEED, -TANK_SPIN_SPEED), (0, 0), (-TANK_SPIN_SPEED, TANK_SPIN_SPEED)],
    [(-TANK_TURN_SPEED, -TANK_FULL_SPEED), (-TANK_FULL_SPEED, -TANK_FULL_SPEED), (-TANK_FULL_SPEED, -TANK_TURN_SPEED)],
];

const TANK_BANK: usize = 1;
const TANK_LEFT_TERMINAL: Terminal = Terminal::A;
const TANK_RIGHT_TERMINAL: Terminal = Terminal::C;
const TANK_STICK_X: gilrs::Axis = gilrs::Axis::LeftStickX;
const TANK_STICK_Y: gilrs::Axis = gilrs::Axis::LeftStickY;

const BODY_STICK_X: gilrs::Axis = gilrs::Axis::RightStickX;
const BODY_STICK_Y: gilrs::Axis = gilrs::Axis::RightStickY;

const BODY_BANK: usize = 1;
const BODY_TERMINAL: Terminal = Terminal::B;
const BODY_TILT_SPEED: f32 = 255.0;
const BODY_UP_BUTTON: gilrs::Button = gilrs::Button::RightTrigger;
const BODY_DOWN_BUTTON: gilrs::Button = gilrs::Button::RightTrigger2;

const HEAD_BANK: usize = 0;
const HEAD_TERMINAL: Terminal = Terminal::A;
const HEAD_TURN_SPEED: f32 = 100.0;
const HEAD_LEFT_BUTTON: gilrs::Button = gilrs::Button::West;
const HEAD_RIGHT_BUTTON: gilrs::Button = gilrs::Button::South;

const CLAW_BANK: usize = 0;
const CLAW_TERMINAL: Terminal = Terminal::C;
const CLAW_SPEED: f32 = 255.0;
const OPEN_CLAW_BUTTON: gilrs::Button = gilrs::Button::LeftTrigger;
const CLOSE_CLAW_BUTTON: gilrs::Button = gilrs::Button::LeftTrigger2;

const RECORD_BUTTON: gilrs::Button = gilrs::Button::Select;
const PLAY_BUTTON: gilrs::Button = gilrs::Button::Start;

pub async fn run(enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // create app and run it
    let app = App::new("Crossterm Demo", enhanced_graphics);
    let app_result = run_app(&mut terminal, app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn to_i16(val: f32) -> i16 {
    val as i16
}

async fn handle_button_change(button: gilrs::Button, value: f32, app: &mut App<'_>, cubes: &mut [CircuitCube]) {
    match button {
        HEAD_LEFT_BUTTON => {
            let power = to_i16(HEAD_TURN_SPEED * -value);
            cubes[HEAD_BANK].send_cmd(Command::SetPower(HEAD_TERMINAL, power)).await;
            app.log_debug(format!("HEAD_LEFT -- power: {power}"));
        }
        HEAD_RIGHT_BUTTON => {
            let power = to_i16(HEAD_TURN_SPEED * value);
            cubes[HEAD_BANK].send_cmd(Command::SetPower(HEAD_TERMINAL, power)).await;
            app.log_debug(format!("HEAD_RIGHT -- power: {power}"));
        }

        BODY_UP_BUTTON => {
            let power = to_i16(BODY_TILT_SPEED * -value);
            cubes[BODY_BANK].send_cmd(Command::SetPower(BODY_TERMINAL, power)).await;
            app.log_debug(format!("BODY_UP -- power: {power}"));
        }
        BODY_DOWN_BUTTON => {
            let power = to_i16(BODY_TILT_SPEED * value);
            cubes[BODY_BANK].send_cmd(Command::SetPower(BODY_TERMINAL, power)).await;
            app.log_debug(format!("BODY_DOWN -- power: {power}"));
        }

        OPEN_CLAW_BUTTON => {
            let power = to_i16(CLAW_SPEED * -value);
            cubes[CLAW_BANK].send_cmd(Command::SetPower(CLAW_TERMINAL, power)).await;
            app.log_debug(format!("OPEN_CLAW -- power: {power}"));
        }
        CLOSE_CLAW_BUTTON => {
            let power = to_i16(CLAW_SPEED * value);
            cubes[CLAW_BANK].send_cmd(Command::SetPower(CLAW_TERMINAL, power)).await;
            app.log_debug(format!("CLOSE_CLAW -- power: {power}"));
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
            app.log_debug(format!("unhandled button: {:?} with value {:}", button, value));
        }
    }
}

#[derive(Debug)]
struct TwoAxisState {
    pub val_x: f32,
    pub val_y: f32,
}

fn get_tread_speed(axis_state: &TwoAxisState) -> (i16, i16) {
    let x = 1.0 + axis_state.val_x.round();
    let y = 1.0 + axis_state.val_y.round();

    TANK_POWER_MAP[y as usize][x as usize]
}

async fn handle_axis_change(axis: gilrs::Axis, value: f32, tank_state: &mut TwoAxisState, body_state: &mut TwoAxisState, app: &mut App<'_>, cubes: &mut [CircuitCube]) {
    let mut updated_tank_state: bool = false;
    let mut updated_body_state: bool = false;
    match axis {
        TANK_STICK_X => {
            tank_state.val_x = value;
            updated_tank_state = true;
        }
        TANK_STICK_Y => {
            tank_state.val_y = value;
            updated_tank_state = true;
        }
        BODY_STICK_X => {
            tank_state.val_x = value;
            updated_body_state = true;
        }
        BODY_STICK_Y => {
            tank_state.val_y = value;
            updated_body_state = true;
        }
        _ => {
            app.log_debug(format!("unhandled axis: {:?}", axis));
        }
    }

    if updated_tank_state {
        let speeds = get_tread_speed(tank_state);

        cubes[TANK_BANK].send_cmd(circuitcubes::Command::SetPower(TANK_LEFT_TERMINAL, speeds.0)).await;
        cubes[TANK_BANK].send_cmd(circuitcubes::Command::SetPower(TANK_RIGHT_TERMINAL, speeds.1)).await;

        // app.log_debug(format!("TANK axis: {:?}", axis));
    } else if updated_body_state {
        app.log_debug(format!("BODY axis: {:?}", axis));
    }
}

async fn run_app<B: Backend>(terminal: &mut ratatui::Terminal<B>, mut app: App<'_>) -> Result<(), Box<dyn Error>> {
    let mut cubes = get_all_cubes().await;

    app.log_debug(format!("found {} circuit cubes!", cubes.len()));
    for cube in &cubes {
        app.log_debug(format!("cube {}", cube.get_addr()));
    }

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    // for (_id, gamepad) in gilrs.gamepads() {
    //     println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    // }

    let mut active_gamepad = None;
    let mut tank_state = TwoAxisState { val_x: 0.0, val_y: 0.0 };
    let mut body_state = TwoAxisState { val_x: 0.0, val_y: 0.0 };

    'running: loop {
        // Examine new events
        while let Some(gilrs::Event { id, event, time, .. }) = gilrs.next_event() {
            active_gamepad = Some(id);
            match event {
                EventType::ButtonPressed(..) => (),
                EventType::ButtonReleased(..) => (),
                EventType::ButtonChanged(button, value, ..) => handle_button_change(button, value, &mut app, &mut cubes).await,
                EventType::AxisChanged(axis, value, ..) => {
                    handle_axis_change(axis, value, &mut tank_state, &mut body_state, &mut app, &mut cubes).await;
                }
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
