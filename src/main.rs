use std::process::Command;
use std::time::Duration;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::widgets::{Block, Borders, Paragraph};
use tui::style::{Color, Style};
use crossterm::event::{self, Event as CEvent, KeyCode};
use tokio::sync::mpsc;
use tokio::time::interval;
use std::env;

enum Event<I> {
    Input(I),
    Tick,
    Quit,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <interval_seconds> <command>", args[0]);
        return Ok(());
    }

    let interval_secs: u64 = args[1].parse().expect("Invalid interval");
    let command = &args[2];

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::channel(100);
    let interval_duration = Duration::from_secs(interval_secs);

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut interval = interval(interval_duration);
        loop {
            interval.tick().await;
            if tx_clone.send(Event::Tick).await.is_err() {
                break;
            }
        }
    });

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    if key.code == KeyCode::Char('q') {
                        let _ = tx_clone.send(Event::Quit).await;
                        break;
                    } else {
                        if tx_clone.send(Event::Input(key)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                match event {
                    Event::Input(event) => {
                        if event.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                    Event::Tick => {
                        let output = Command::new("zsh")
                            .arg("-c")
                            .arg(command)
                            .output()
                            .expect("Failed to execute command");
                        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

                        terminal.draw(|f| {
                            let size = f.size();
                            let block = Block::default()
                                .title("Watch Command")
                                .borders(Borders::ALL);
                            let paragraph = Paragraph::new(output_str.clone())
                                .block(block)
                                .style(Style::default().fg(Color::White));
                            f.render_widget(paragraph, size);
                        })?;
                    }
                    Event::Quit => {
                        break;
                    }
                }
            }
        }
    }

    terminal.show_cursor()?;
    terminal.clear()?;
    Ok(())
}
