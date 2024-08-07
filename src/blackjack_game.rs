use blackjack_rust::blackjack::{deck::Card, traits::Stringable};
use blackjack_rust::blackjack::play_blackjack_hand::HandResult;
use blackjack_rust::blackjack::hand::BlackjackHand;
use blackjack_rust::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;

use std::{
    io::{self, stdout},
    str::FromStr,
};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    text::Span,
    widgets::*,
};

use blackjack_rust::game::channel_game::{get_short_letter, get_word, GameAction, GameInfo};
use blackjack_rust::game::sync_game::SyncGame;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let (action_sender, action_receiver) = mpsc::sync_channel::<GameAction>(1);
    let (option_sender, option_receiver) = mpsc::sync_channel::<Vec<GameAction>>(1);
    let (game_info_sender, game_info_receiver) = mpsc::sync_channel::<GameInfo>(1);
    let option_sender_clone = option_sender.clone();
    let options = Arc::new(Mutex::new(None));
    let game_info = Arc::new(Mutex::new(None));
    let t = thread::spawn(move || {
        let mut sync_game = SyncGame::new(
            action_receiver,
            option_sender_clone,
            game_info_sender,
            false,
        );
        loop {
            sync_game.play();
            if !sync_game.ask_to_play_another_hand() {
                break;
            }
        }
        sync_game.cleanup();
    });
    let mut should_quit = false;
    while !should_quit {
        match option_receiver.try_recv() {
            Ok(options_received) => {
                let mut opt = options.lock().unwrap();
                *opt = Some(options_received);
            }
            Err(_message) => {}
        }

        match game_info_receiver.try_recv() {
            Ok(game_info_received) => {
                let mut game = game_info.lock().unwrap();
                *game = Some(game_info_received);
            }
            Err(_message) => {}
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

fn create_centered_text_from_hand(hand: &Vec<Card>) -> Line {
    let mut spans = Vec::<Span>::new();
    // Iterate over the cards of the player's hand
    for card in hand {
        // Get the color from the suit
        let color = Color::from_str(&card.suit().get_color()).expect("Invalid color");

        // Create a styled Span for the card
        let styled_span = Span::styled(
            card.to_sonderzeichen(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        );

        // Push the styled Span and a space Span to the Text object
        spans.push(styled_span);
        spans.push(Span::raw(" "));
    }


    let blackjack_hand = BlackjackHand::new(hand);
    let points = evaluate_blackjack_hand(&blackjack_hand);
    let points_string = points.to_ui();

    spans.push(Span::raw("(".to_owned() + points_string.as_str() + ")"));

    // make sure the text renders in one line
    Line::from(spans)
}

fn get_bet_background_color(hand_result: Option<HandResult>) -> Color {
    match hand_result {
        Some(HandResult::Win(_)) => Color::Green,
        Some(HandResult::Loss(_)) => Color::Red,
        Some(HandResult::Tie) => Color::Yellow,
        None => Color::Reset,
    }
}

fn draw_ui(frame: &mut Frame, game_info: Option<GameInfo>, options: Option<Vec<GameAction>>) {
    let mut main_constraints = vec![Constraint::Length(1)];
    if let Some(game_info) = &game_info {
        for _ in 0..game_info.hands.len() {
            main_constraints.push(Constraint::Min(0));
        }
    } else {
        main_constraints.push(Constraint::Min(0));
    }
    main_constraints.push(Constraint::Min(0));
    main_constraints.push(Constraint::Min(0));
    let main_layout =
        Layout::new(Direction::Vertical, main_constraints.as_slice()).split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Blackjack Game"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("Available Actions"),
        main_layout[main_constraints.len() - 1],
    );

    let money_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[main_constraints.len() - 2]);

    let options_clone = options.clone();
    match options_clone {
        None => {
            Layout::default().split(main_layout[main_constraints.len() - 1]);
        }
        Some(options) => {
            let options_percentage = 100 / options.len() as u16;
            let options_layout = Layout::new(
                Direction::Horizontal,
                options
                    .iter()
                    .map(|_| Constraint::Percentage(options_percentage))
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .split(main_layout[main_constraints.len() - 1]);
            for (i, option) in options.iter().enumerate() {
                let block = Block::bordered().title(get_word(*option));
                frame.render_widget(block.clone(), options_layout[i]);
                frame.render_widget(
                    Paragraph::new("Press ".to_owned() + &get_short_letter(*option)),
                    block.inner(options_layout[i]),
                );
            }
        }
    };

    if let Some(game_info) = game_info {
        for (i, hand) in game_info.hands.iter().enumerate() {
            let hand_layout = Layout::new(
                Direction::Horizontal,
                [Constraint::Percentage(50), Constraint::Percentage(50)],
            )
            .split(main_layout[i + 1]);

            let hand_box = Block::bordered().title(format!("Hand {}", i + 1));
            let bet_box = Block::bordered().title("Bet").bg(get_bet_background_color(hand.result.clone()));
            frame.render_widget(hand_box.clone(), hand_layout[0]);
            frame.render_widget(bet_box.clone(), hand_layout[1]);

            // iterate over the cards of the player hand and make the string of each card created by to_sonderzeichen() to the colour of suit.get_color()
            let cards = hand.player_hand.get_cards();
            let text = create_centered_text_from_hand(&cards);

            frame.render_widget(text, hand_box.inner(hand_layout[0]));

            frame.render_widget(
                Paragraph::new("$".to_owned() + &hand.player_bet.to_string()),
                bet_box.inner(hand_layout[1]),
            );
        }

        let dealer_hand = Block::bordered().title("Dealer hand");
        frame.render_widget(dealer_hand.clone(), money_layout[1]);

        let dealer_cards = game_info
            .dealer_hand
            .get_cards(!game_info.current_hand_finished);
        let dealer_text = create_centered_text_from_hand(&dealer_cards);

        frame.render_widget(dealer_text, dealer_hand.inner(money_layout[1]));

        let your_money = Block::bordered().title("Your money");
        frame.render_widget(your_money.clone(), money_layout[0]);

        frame.render_widget(
            Paragraph::new("$".to_owned() + &game_info.current_balance.to_string()),
            your_money.inner(money_layout[0]),
        );
    }
}
