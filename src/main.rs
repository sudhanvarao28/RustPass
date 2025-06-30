use argon2::password_hash::Value;
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
pub mod sleddb;
use std::{any, collections::HashMap, default, io::{self, stdout}, path::Prefix};

use crate::encrypt_decrypt::encrypt;

enum Screen {
    FirstSetup,
    Login,
    Menu,
    AddKeyEntry,
    AddPasswordEntry,
    ViewPassword,
}

fn add_entry(master_password:&str, key: &str, value:&str) -> Result<(),anyhow::Error>{

    let output = encrypt(&value.as_bytes(), &master_password.as_bytes())?;
    sleddb::insert(key, &output)?;
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut screen = if encrypt_decrypt::is_master_password_configured()? {
        Screen::Login
    } else {
        Screen::FirstSetup
    };
    
    let mut masterpass_input = String::new();
    let mut password_input = String::new();
    let mut key_input = String::new();
    let menu_items = vec!["Add Password", "View Password", "Exit"];
    let mut selected = 0;
    let mut stored_passwords: HashMap<String, Vec<u8>>= HashMap::new();
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
                Screen::AddPasswordEntry =>{
                    let input_block = Paragraph::new(password_input.as_str())
                                                    .block(Block::default().title("Enter Password for Key").borders(Borders::ALL));

                    f.render_widget(input_block, size);
                },
                Screen::AddKeyEntry => {
                    let input_block = Paragraph::new(key_input.as_str())
                                                    .block(Block::default().title("Enter Key").borders(Borders::ALL));

                    f.render_widget(input_block, size);
                }

                Screen::FirstSetup => {
                    let block = Paragraph::new(masterpass_input.as_str())
                        .block(Block::default().title("Set New Master Password").borders(Borders::ALL));
                    f.render_widget(block, size);
                },
            
                Screen::Login => {
                    let block = Paragraph::new(masterpass_input.as_str())
                        .block(Block::default().title("Enter Master Password To Login").borders(Borders::ALL));
                    f.render_widget(block, size);
                }
                Screen::ViewPassword =>{
                    let items: Vec<ListItem> = stored_passwords.iter().map(|(key, value)| {
                        let value_str = String::from_utf8_lossy(value);
                        ListItem::new(format!("{} : {}", key,value_str))
                    }).collect();
                    let list = List::new(items)
                    .block(Block::default().title("Stored Passwords").borders(Borders::ALL));
            
                    f.render_widget(list, size);
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
                                password_input.clear();
                                screen = Screen::AddKeyEntry;
                            },
                            1 => {
                                screen = Screen::ViewPassword;
                            },
                            2 => break,
                            _ => {}

                        },
                        _ => {}
                    }
                },
                Screen::AddKeyEntry =>{
                    match key.code {
                        KeyCode::Esc => screen = Screen::Menu,
                        KeyCode::Char(q) => {
                            key_input.push(q);
                        },
                        KeyCode::Backspace => {
                            key_input.pop();
                        },
                        KeyCode::Enter => {
                            screen = Screen::AddPasswordEntry;
                        }
                        _ => {}
                    }
                },
               
                Screen::AddPasswordEntry =>{
                    match key.code {
                        KeyCode::Esc => screen = Screen::Menu,
                        KeyCode::Char(q) => {
                            password_input.push(q);
                        },
                        KeyCode::Backspace => {
                            password_input.pop();
                        },
                        KeyCode::Enter => {
                            add_entry(&masterpass_input, &key_input.as_str(), &password_input.as_str())?;
                            key_input.clear();
                            password_input.clear();
                            screen = Screen::Menu;
                        }
                        _ => {}
                    }
                },
                Screen::FirstSetup => {
                    match key.code{
                        KeyCode::Esc => {
                            masterpass_input.clear();
                            screen = Screen::Menu;
                        },
                        KeyCode::Enter => {
                            encrypt_decrypt::store_master_password(&masterpass_input.as_bytes())?;
                            screen = Screen::Menu
                        },
                        KeyCode::Backspace => {
                            masterpass_input.pop();
                        },
                        KeyCode::Char(q) => {
                            masterpass_input.push(q);
                        },
                        _ => {}
                        
                    }
                },
                Screen::Login => {
                    match key.code {
                        KeyCode::Enter => {
                            if !encrypt_decrypt::verify_master_password(&masterpass_input.as_bytes())?{
                                return Err(anyhow::anyhow!("\nWrong Password\n"));
                            }
                            screen = Screen::Menu;
                        }
                        KeyCode::Esc => {
                            masterpass_input.clear();
                            screen = Screen::Menu;
                        }
                        KeyCode::Backspace => {
                            masterpass_input.pop();
                        }
                        KeyCode::Char(c) => {
                            masterpass_input.push(c);
                        }
                        _ => {}
                    }
                },
                Screen::ViewPassword => {
                    stored_passwords = sleddb::iter_get_passwords(masterpass_input.as_bytes())?;
                    match key.code {
                        KeyCode::Esc => {
                            stored_passwords.clear();
                            screen = Screen::Menu;
                        },
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


// Add entry success screen when you have added a new entry