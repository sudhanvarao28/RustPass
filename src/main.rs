use crossterm::{event::{self, Event, KeyCode},execute,
terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

use ratatui::{
    backend::{self, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

pub mod encrypt_decrypt;
use std::{any, io::{self, stdout}, path::Prefix};

struct Userdata{
    website: String,
    uname: String,
    password: String,
}
enum Screen {
    Menu,
    AddEntry,
}
fn main() -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut screen = Screen::Menu;
    let mut input = String::new();
    let menu_items = vec!["Add Password", "View Password", "Exit"];
    let mut selected = 0;

    loop{
        terminal.draw(|f|{
            let size = f.area();

            match screen {
                Screen::Menu => {
                    let items: Vec<_>= menu_items.iter().enumerate().map(|(i,&item)|{
                        ListItem::new(item).style(if i == selected {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        })
                        }).collect();
                    
                    let list = List::new(items)
                                        .block(Block::default()
                                        .title("RustPass Menu")
                                        .borders(Borders::ALL));

                    f.render_widget(list, size);

                    },
                Screen::AddEntry =>{
                    let input_block = Paragraph::new(input.as_str())
                                                    .block(Block::default().title("Add new password").borders(Borders::ALL));

                    f.render_widget(input_block, size);
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match screen {
                Screen::Menu =>{
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Up => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        },
                        KeyCode::Down => {
                            if selected < menu_items.len() - 1 {
                                selected += 1;
                            }
                        },
                        KeyCode::Enter => match selected {
                            0 => {
                                input.clear();
                                screen = Screen::AddEntry;
                            },
                            1 => println!("View Entries not implemented"),
                            2 => break,
                            _ => {}

                        },
                        _ => {}
                    }
                },
                Screen::AddEntry =>{
                    match key.code {
                        KeyCode::Esc => screen = Screen::Menu,
                        _ => {}
                    }
                }
            }
        }

    }
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
    
}
