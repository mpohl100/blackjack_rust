use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
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
        Block::new().borders(Borders::TOP).title("Available Actions"),
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
    ).split(main_layout[2]);

    let your_hand = Block::bordered().title("Your hand");
    let dealer_hand = Block::bordered().title("Dealer hand");
    frame.render_widget(your_hand.clone(), hands_layout[0]);
    frame.render_widget(dealer_hand.clone(), hands_layout[1]);

    let your_money = Block::bordered().title("Your money");
    let your_bet = Block::bordered().title("Bet"); 
    frame.render_widget(your_money.clone(), money_layout[0]);
    frame.render_widget(your_bet.clone(), money_layout[1]);

    frame.render_widget(Paragraph::new("Ad 2h"), your_hand.inner(hands_layout[0]));
    frame.render_widget(Paragraph::new("Kd *"), dealer_hand.inner(hands_layout[1]));

    frame.render_widget(Paragraph::new("$1000"), your_money.inner(money_layout[0]));
    frame.render_widget(Paragraph::new("$1"), your_bet.inner(money_layout[1]));
}