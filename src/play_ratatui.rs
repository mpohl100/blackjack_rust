use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*, TerminalOptions,
};

use blackjack_rust::game::channel_game::{ChannelGame, GameAction, GameInfo, get_word, get_short_letter};

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let (action_sender, action_receiver) = mpsc::channel::<GameAction>(32);
    let (option_sender, mut option_receiver) = mpsc::channel::<Vec<GameAction>>(32);
    let option_sender_clone = option_sender.clone();
    let channel_game = Arc::new(Mutex::new(ChannelGame::new(action_receiver, option_sender_clone, false).await));
    let channel_game_clone = channel_game.clone();
    let t = tokio::spawn(async move {
        loop {
            channel_game_clone.lock().await.play().await;
            if !channel_game_clone.lock().await.ask_to_play_another_hand().await {
                break;
            }
        }
    });
    let mut options = Some(option_receiver.recv().await.unwrap());
    while !should_quit {
        if options.is_none() {
            options = Some(option_receiver.recv().await.unwrap());
        }
        let game_info = channel_game.lock().await.get_game_info().await;

        let options_clone = options.clone().unwrap();
        let ui = move |frame: &mut Frame| {
            draw_ui(frame, game_info, options_clone);
        };
        terminal.draw(ui)?;
        let choice = handle_events()?;
        action_sender.send(choice).await.unwrap();
        
        if choice == GameAction::Stop {
            should_quit = true;
        }

        if choice != GameAction::Continue {
            options = None;
        }
    }

    t.await.unwrap();
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<GameAction> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code{
                    KeyCode::Char('q') => return Ok(GameAction::Stop),
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

fn draw_ui(frame: &mut Frame, game_info: GameInfo, options: Vec<GameAction>) {
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

    let options_percentage = 100 / options.len() as u16;
    let options_layout = Layout::new(
        Direction::Horizontal,
        options.iter().map(|_| Constraint::Percentage(options_percentage)).collect::<Vec<_>>().as_slice(),
    ).split(main_layout[3]);

    let your_hand = Block::bordered().title("Your hand");
    let dealer_hand = Block::bordered().title("Dealer hand");
    frame.render_widget(your_hand.clone(), hands_layout[0]);
    frame.render_widget(dealer_hand.clone(), hands_layout[1]);

    let your_money = Block::bordered().title("Your money");
    let your_bet = Block::bordered().title("Bet");
    frame.render_widget(your_money.clone(), money_layout[0]);
    frame.render_widget(your_bet.clone(), money_layout[1]);

    for (i, option) in options.iter().enumerate() {
        frame.render_widget(
            Block::bordered().title(get_word(*option)),
            options_layout[i],
        );
    }

    frame.render_widget(Paragraph::new(game_info.player_hand.to_string_internal()), your_hand.inner(hands_layout[0]));
    frame.render_widget(Paragraph::new(game_info.dealer_hand.to_string_internal(true)), dealer_hand.inner(hands_layout[1]));

    frame.render_widget(Paragraph::new("$".to_owned() + &game_info.current_balance.to_string()), your_money.inner(money_layout[0]));
    frame.render_widget(Paragraph::new("$".to_owned() + &game_info.player_bet.to_string()), your_bet.inner(money_layout[1]));

    for (i, option) in options.iter().enumerate() {
        frame.render_widget(
            Paragraph::new("Press ".to_owned() + &get_short_letter(*option)),
            options_layout[i],
        );
    }
}
