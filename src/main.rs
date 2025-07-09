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
use std::{any, char, collections::HashMap, default, io::{self, stdout}, path::Prefix};

use crate::encrypt_decrypt::encrypt;

enum Screen {
    FirstSetup,
    Login,
    Menu,
    AddKeyEntry,
    AddPasswordEntry,
    EditKey,
    EditPassword,
    DeleteKey,
    ViewPassword,
}

#[derive(Default)]
struct Inputs {
    masterpass_input: String,
    password_input: String,
    key_input: String,
    edit_key: String,
    edit_password: String,
    delete_key: String,
    stored_passwords: HashMap<String, Vec<u8>>
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
    
    let mut input = Inputs::default();
    let menu_items = vec!["Add Password", "View Password", "Edit Password", "Delete Password", "Exit"];
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
                Screen::AddPasswordEntry =>{
                    let input_block = Paragraph::new(input.password_input.as_str())
                                                    .block(Block::default().title("Enter Password for Key").borders(Borders::ALL));

                    f.render_widget(input_block, size);
                },
                Screen::AddKeyEntry => {
                    let input_block = Paragraph::new(input.key_input.as_str())
                                                    .block(Block::default()
                                                    .title("Enter Key")
                                                    .borders(Borders::ALL));

                    f.render_widget(input_block, size);
                }

                Screen::FirstSetup => {
                    let block = Paragraph::new(input.masterpass_input.as_str())
                        .block(Block::default()
                        .title("Set New Master Password")
                        .borders(Borders::ALL));
                    f.render_widget(block, size);
                },
            
                Screen::Login => {
                    let block = Paragraph::new(input.masterpass_input.as_str())
                        .block(Block::default()
                        .title("Enter Master Password To Login")
                        .borders(Borders::ALL));
                    f.render_widget(block, size);
                },
                Screen::ViewPassword =>{
                    let items: Vec<ListItem> = input.stored_passwords.iter().map(|(key, value)| {
                        let value_str = String::from_utf8_lossy(value);
                        ListItem::new(format!("{} : {}", key,value_str))
                    }).collect();
                    let list = List::new(items)
                    .block(Block::default().title("Stored Passwords").borders(Borders::ALL));
            
                    f.render_widget(list, size);
                },
                Screen::EditKey => {
                    let block = Paragraph::new(input.edit_key.as_str())
                    .block(Block::default()
                    .title("Enter key to edit")
                    .borders(Borders::all()));
                f.render_widget(block, size);
                },
                Screen::EditPassword => {
                    let block = Paragraph::new(input.edit_password.as_str())
                    .block(Block::default()
                    .title("Enter new edited password")
                    .borders(Borders::all()));
                f.render_widget(block, size);
                },
                Screen::DeleteKey => {
                    let block = Paragraph::new(input.delete_key.as_str())
                    .block(Block::default()
                    .title("Enter key to delete")
                    .borders(Borders::all()));
                f.render_widget(block, size);
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
                                input.password_input.clear();
                                screen = Screen::AddKeyEntry;
                            },
                            1 => {
                                screen = Screen::ViewPassword;
                            },
                            2 => {
                                screen = Screen::EditKey;
                            },
                            3 => {
                                screen = Screen::DeleteKey;
                            },
                            4 => break,
                            _ => {}

                        },
                        _ => {}
                    }
                },
                Screen::AddKeyEntry =>{
                    match key.code {
                        KeyCode::Esc => screen = Screen::Menu,
                        KeyCode::Char(q) => {
                            input.key_input.push(q);
                        },
                        KeyCode::Backspace => {
                            input.key_input.pop();
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
                            input.password_input.push(q);
                        },
                        KeyCode::Backspace => {
                            input.password_input.pop();
                        },
                        KeyCode::Enter => {
                            add_entry(&input.masterpass_input, &input.key_input.as_str(), &input.password_input.as_str())?;
                            input.key_input.clear();
                            input.password_input.clear();
                            screen = Screen::Menu;
                        }
                        _ => {}
                    }
                },
                Screen::FirstSetup => {
                    match key.code{
                        KeyCode::Esc => {
                            input.masterpass_input.clear();
                            screen = Screen::Menu;
                        },
                        KeyCode::Enter => {
                            encrypt_decrypt::store_master_password(&input.masterpass_input.as_bytes())?;
                            screen = Screen::Menu
                        },
                        KeyCode::Backspace => {
                            input.masterpass_input.pop();
                        },
                        KeyCode::Char(q) => {
                            input.masterpass_input.push(q);
                        },
                        _ => {}
                        
                    }
                },
                Screen::Login => {
                    match key.code {
                        KeyCode::Enter => {
                            if !encrypt_decrypt::verify_master_password(&input.masterpass_input.as_bytes())?{
                                return Err(anyhow::anyhow!("\nWrong Password\n"));
                            }
                            screen = Screen::Menu;
                        }
                        KeyCode::Esc => {
                            input.masterpass_input.clear();
                            screen = Screen::Menu;
                        }
                        KeyCode::Backspace => {
                            input.masterpass_input.pop();
                        }
                        KeyCode::Char(c) => {
                            input.masterpass_input.push(c);
                        }
                        _ => {}
                    }
                },
                Screen::ViewPassword => {
                    input.stored_passwords = sleddb::iter_get_passwords(input.masterpass_input.as_bytes())?;
                    match key.code {
                        KeyCode::Esc => {
                            input.stored_passwords.clear();
                            screen = Screen::Menu;
                        },
                        _ => {}
                    }
                },
                Screen::EditKey => {
                    match key.code {
                        KeyCode::Esc => {
                            input.edit_key.clear();
                            input.edit_password.clear();
                            screen = Screen::Menu;
                        },
                        KeyCode::Enter => {
                            if let Some(_) = sleddb::get(&input.edit_key) {
                                screen = Screen::EditPassword;
                                } else {
                                    screen = Screen::Menu
                                }
                        },
                        KeyCode::Char(c) => {
                            input.edit_key.push(c);
                        },
                        KeyCode::Backspace => {
                            input.edit_key.pop();
                        },
                        _ => {}
                    }
                },
                Screen::EditPassword => {
                    match key.code {
                        KeyCode::Esc => {
                            input.edit_key.clear();
                            input.edit_password.clear();
                            screen = Screen::Menu;
                        },
                        KeyCode::Enter => {
                            add_entry(&input.masterpass_input, &input.edit_key.as_str(), &input.edit_password.as_str())?;
                            input.edit_key.clear();
                            input.edit_password.clear();
                            screen = Screen::Menu;
                        },
                        KeyCode::Char(c) => {
                            input.edit_password.push(c);
                        },
                        KeyCode::Backspace => {
                            input.edit_password.pop();
                        },
                        _ => {}
                    }
                },
                Screen::DeleteKey => {
                    match key.code {
                        KeyCode::Esc => {
                            screen = Screen::Menu;
                        },
                        KeyCode::Char(c) => {
                            input.delete_key.push(c);
                        },
                        KeyCode::Backspace => {
                            input.delete_key.pop();
                        },
                        KeyCode::Enter => {
                            sleddb::remove(&input.delete_key)?;
                            input.delete_key.clear();
                            screen = Screen::Menu;
                        }
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