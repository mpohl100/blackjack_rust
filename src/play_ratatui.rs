use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
    TerminalOptions,
};

use blackjack_rust::game::sync_game::SyncGame;
use blackjack_rust::game::{
    self,
    channel_game::{get_short_letter, get_word, GameAction, GameInfo},
};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let (action_sender, action_receiver) = mpsc::sync_channel::<GameAction>(1);
    let (option_sender, mut option_receiver) = mpsc::sync_channel::<Vec<GameAction>>(1);
    let (game_info_sender, game_info_receiver) = mpsc::sync_channel::<GameInfo>(1);
    let option_sender_clone = option_sender.clone();
    let mut options = Arc::new(Mutex::new(None));
    let mut game_info = Arc::new(Mutex::new(None));
    let options_clone = options.clone();
    let game_info_clone = game_info.clone();
    let reset_data = move || {
        let mut opt = options_clone.lock().unwrap();
        *opt = None;
        let mut game = game_info_clone.lock().unwrap();
        *game = None;
    };
    let t = thread::spawn(move || {
        let sync_game = Arc::new(Mutex::<SyncGame>::new(SyncGame::new(
            action_receiver,
            option_sender_clone,
            game_info_sender,
            false,
        )));
        loop {
            sync_game.lock().unwrap().play();
            if !sync_game.lock().unwrap().ask_to_play_another_hand() {
                break;
            }
            reset_data();
        }
    });
    let mut should_quit = false;
    while !should_quit {
        
        if options.lock().unwrap().is_none() {
            let mut opt = options.lock().unwrap();
            match option_receiver.try_recv() {
                Ok(options_received) => {
                    *opt = Some(options_received);
                }
                Err(_message) => {}
            }
        }
        if game_info.lock().unwrap().is_none() {
            let mut game = game_info.lock().unwrap();
            match game_info_receiver.try_recv() {
                Ok(game_info_received) => {
                    *game = Some(game_info_received);
                }
                Err(_message) => {}
            }
        }

        let options_clone = options.lock().unwrap().clone();
        let game_info_clone = game_info.lock().unwrap().clone();
        let ui = move |frame: &mut Frame| {
            draw_ui(frame, game_info_clone, options_clone);
        };
        terminal.draw(ui)?;

        let choice = handle_events()?;
        if choice != GameAction::Continue {
            action_sender.send(choice).unwrap();
        }

        if choice == GameAction::Stop {
            should_quit = true;
        }

        if choice != GameAction::Continue {
            let mut opt = options.lock().unwrap();
            *opt = None;
            let mut game = game_info.lock().unwrap();
            *game = None;
        }
    }

    t.join().unwrap();
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<GameAction> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('x') => return Ok(GameAction::Stop),
                    KeyCode::Char('h') => return Ok(GameAction::Hit),
                    KeyCode::Char('t') => return Ok(GameAction::Stand),
                    KeyCode::Char('d') => return Ok(GameAction::DoubleDown),
                    KeyCode::Char('s') => return Ok(GameAction::Split),
                    _ => return Ok(GameAction::Continue),
                }
            }
        }
    }
    Ok(GameAction::Continue)
}

fn draw_ui(frame: &mut Frame, game_info: Option<GameInfo>, options: Option<Vec<GameAction>>) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Blackjack Game"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("Available Actions"),
        main_layout[3],
    );

    let hands_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[1]);

    let money_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[2]);

    let options_clone = options.clone();
    let options_layout = match options_clone {
        None => Layout::default().split(main_layout[3]),
        Some(options) => {
            let options_percentage = 100 / options.len() as u16;
            Layout::new(
                Direction::Horizontal,
                options
                    .iter()
                    .map(|_| Constraint::Percentage(options_percentage))
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .split(main_layout[3])
        }
    };
    let your_hand = Block::bordered().title("Your hand");
    let dealer_hand = Block::bordered().title("Dealer hand");
    frame.render_widget(your_hand.clone(), hands_layout[0]);
    frame.render_widget(dealer_hand.clone(), hands_layout[1]);

    let your_money = Block::bordered().title("Your money");
    let your_bet = Block::bordered().title("Bet");
    frame.render_widget(your_money.clone(), money_layout[0]);
    frame.render_widget(your_bet.clone(), money_layout[1]);

    let options_clone_2 = options.clone();
    if !options_clone_2.is_none() {
        for (i, option) in options_clone_2.unwrap().iter().enumerate() {
            frame.render_widget(
                Block::bordered().title(get_word(*option)),
                options_layout[i],
            );
        }
    }

    if !game_info.is_none() {
        let game_info_unwrapped = game_info.unwrap();
        frame.render_widget(
            Paragraph::new(game_info_unwrapped.player_hand.to_string_internal()),
            your_hand.inner(hands_layout[0]),
        );
        frame.render_widget(
            Paragraph::new(game_info_unwrapped.dealer_hand.to_string_internal(true)),
            dealer_hand.inner(hands_layout[1]),
        );

        frame.render_widget(
            Paragraph::new("$".to_owned() + &game_info_unwrapped.current_balance.to_string()),
            your_money.inner(money_layout[0]),
        );
        frame.render_widget(
            Paragraph::new("$".to_owned() + &game_info_unwrapped.player_bet.to_string()),
            your_bet.inner(money_layout[1]),
        );
    }

    if !options.is_none() {
        for (i, option) in options.unwrap().iter().enumerate() {
            frame.render_widget(
                Paragraph::new("Press ".to_owned() + &get_short_letter(*option)),
                options_layout[i],
            );
        }
    }
}
