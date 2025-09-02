// src/main.rs

mod scripts;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{cell::RefCell, error::Error, io, fs, process::Command, os::unix::fs::PermissionsExt, rc::Rc};

// A category for each script to control execution order.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScriptCategory {
    Repository,
    General,
}

// A struct to hold all info about a selected item.
struct SelectedItem {
    name: String,
    script_fn: fn() -> &'static str,
    category: ScriptCategory,
}

/// Represents a node in the menu tree. It can be a selectable item or a sub-menu.
pub enum MenuNode {
    Item {
        name: String,
        script_fn: fn() -> &'static str,
        selected: bool,
        category: ScriptCategory,
    },
    Menu {
        name: String,
        children: Vec<Rc<RefCell<MenuNode>>>,
    },
}

impl MenuNode {
    /// Recursively collects detailed info about all selected items.
    fn get_selected_items_info(&self, items: &mut Vec<SelectedItem>) {
        match self {
            MenuNode::Item { name, selected, script_fn, category, .. } => {
                if *selected {
                    items.push(SelectedItem {
                        name: name.clone(),
                        script_fn: *script_fn,
                        category: *category,
                    });
                }
            }
            MenuNode::Menu { children, .. } => {
                for child in children {
                    child.borrow().get_selected_items_info(items);
                }
            }
        }
    }
}


/// Enum to represent the detected Linux distribution.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OsDistribution {
    Rhel,
    Centos,
    Rocky,
    AlmaLinux,
    Unknown,
}

/// Enum to manage the overall state of the application.
enum AppState {
    Running,
    Finished,
    Saving,
}

/// Enum to tell the main function what to do after the TUI exits.
pub enum ActionAfterExit {
    Quit,
    RunScript(String),
}

/// Holds the application's state.
struct App {
    state: AppState,
    menu_tree: Rc<RefCell<MenuNode>>,
    nav_path: Vec<Rc<RefCell<MenuNode>>>,
    selected_index: usize,
    os_distro: OsDistribution,
    reboot_requested: bool,
    filename_input: String,
    save_status_message: Option<String>,
}

fn detect_os() -> OsDistribution {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("ID=") {
                let id = line.trim_start_matches("ID=").trim_matches('"');
                return match id {
                    "rhel" => OsDistribution::Rhel,
                    "centos" => OsDistribution::Centos,
                    "rocky" => OsDistribution::Rocky,
                    "almalinux" => OsDistribution::AlmaLinux,
                    _ => OsDistribution::Unknown,
                };
            }
        }
    }
    OsDistribution::Unknown
}

impl App {
    /// Creates a new App instance with default values.
    fn new() -> App {
        let os_distro = detect_os();
        let menu_tree = scripts::build_menu_tree(os_distro);
        let nav_path = vec![menu_tree.clone()];

        App {
            state: AppState::Running,
            menu_tree,
            nav_path,
            selected_index: 0,
            os_distro,
            reboot_requested: false,
            filename_input: String::new(),
            save_status_message: None,
        }
    }

    /// Generates the shell commands, ensuring repos are first and adding error checks.
    fn generate_commands(&self, reboot: bool) -> String {
        let mut items = Vec::new();
        self.menu_tree.borrow().get_selected_items_info(&mut items);

        // Partition items into categories
        let repos: Vec<&SelectedItem> = items.iter().filter(|i| i.category == ScriptCategory::Repository).collect();
        let general: Vec<&SelectedItem> = items.iter().filter(|i| i.category == ScriptCategory::General).collect();

        let mut command_text = String::new();
        command_text.push_str("#!/bin/bash\n");
        command_text.push_str(&format!("# Generated for {:?} by Enterprise Linux TUI\n\n", self.os_distro));
        
        // Add robust error handling and a logging function
        command_text.push_str("# Exit immediately if a command exits with a non-zero status.\nset -e\n\n");
        command_text.push_str("# Helper for logging steps\nprint_step() {\n    echo\n    echo \"✅ ==> $1\"\n}\n\n");

        if repos.is_empty() && general.is_empty() {
            command_text.push_str("# No options selected.\n");
        }

        // 1. Add repository scripts first
        if !repos.is_empty() {
            command_text.push_str("# --- 1. ENABLING REPOSITORIES ---\n");
            for item in &repos {
                command_text.push_str(&format!("print_step \"{}\"\n", item.name));
                command_text.push_str((item.script_fn)());
                command_text.push_str("\n");
            }
        }

        // 2. Add all other general scripts
        if !general.is_empty() {
            command_text.push_str("\n# --- 2. APPLYING CONFIGURATIONS ---\n");
            for item in &general {
                command_text.push_str(&format!("print_step \"{}\"\n", item.name));
                command_text.push_str((item.script_fn)());
                command_text.push_str("\n");
            }
        }

        if reboot {
            command_text.push_str("\nprint_step \"All tasks complete. Rebooting now...\"\n");
            command_text.push_str("sleep 3\n");
            command_text.push_str("sudo reboot\n");
        } else if !repos.is_empty() || !general.is_empty() {
            command_text.push_str("\nprint_step \"All tasks complete!\"\n");
        }

        command_text
    }
    
    /// Gets just the names of selected items for display in the UI.
    fn get_selected_items(&self) -> Vec<String> {
        let mut items_info = Vec::new();
        self.menu_tree.borrow().get_selected_items_info(&mut items_info);
        items_info.into_iter().map(|i| i.name).collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Ok(ActionAfterExit::RunScript(script_content)) = res {
        let script_path = "/tmp/tui_install_script.sh";
        println!("Saving temporary script to {}...", script_path);
        fs::write(script_path, &script_content)?;
        fs::set_permissions(script_path, fs::Permissions::from_mode(0o755))?;

        println!("Exited TUI. Now attempting to run the script with sudo...");
        println!("--- SCRIPT ---");
        println!("{}", script_content);
        println!("--------------");
        
        let status = Command::new("sudo").arg("bash").arg(script_path).status()?;

        if status.success() {
            println!("\nScript executed successfully.");
        } else {
            println!("\nScript execution failed. Please check the output above.");
        }
        fs::remove_file(script_path)?;
    } else if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<ActionAfterExit> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::Running => {
                    let visible_nodes = get_visible_nodes(&app.nav_path);
                    let visible_len = visible_nodes.len();

                    if visible_len > 0 {
                        app.selected_index = app.selected_index.min(visible_len - 1);
                    } else {
                        app.selected_index = 0;
                    }

                    match key.code {
                        KeyCode::Char('q') => return Ok(ActionAfterExit::Quit),
                        KeyCode::Char('i') => { app.state = AppState::Finished; app.reboot_requested = false; },
                        KeyCode::Char('r') => { app.state = AppState::Finished; app.reboot_requested = true; },
                        KeyCode::Down => {
                            if !visible_nodes.is_empty() {
                                app.selected_index = (app.selected_index + 1) % visible_nodes.len();
                            }
                        }
                        KeyCode::Up => {
                            if !visible_nodes.is_empty() {
                                app.selected_index = (app.selected_index + visible_nodes.len() - 1) % visible_nodes.len();
                            }
                        }
                        KeyCode::Right | KeyCode::Enter => {
                            if let Some((_, selected_rc)) = visible_nodes.get(app.selected_index) {
                                let mut node_mut = selected_rc.borrow_mut();
                                match &mut *node_mut {
                                    MenuNode::Menu { .. } => {
                                        drop(node_mut);
                                        app.nav_path.push(selected_rc.clone());
                                        app.selected_index = 0;
                                    }
                                    MenuNode::Item { selected, .. } => {
                                        *selected = !*selected;
                                    }
                                }
                            }
                        }
                        KeyCode::Left | KeyCode::Backspace => {
                            if app.nav_path.len() > 1 {
                                app.nav_path.pop();
                                app.selected_index = 0;
                            }
                        }
                        _ => {}
                    }
                },
                AppState::Finished => match key.code {
                    KeyCode::Char('q') => return Ok(ActionAfterExit::Quit),
                    KeyCode::Char('s') => app.state = AppState::Saving,
                    KeyCode::Char('r') => return Ok(ActionAfterExit::RunScript(app.generate_commands(app.reboot_requested))),
                    KeyCode::Esc | KeyCode::Backspace => app.state = AppState::Running,
                    _ => {}
                },
                AppState::Saving => match key.code {
                    KeyCode::Char(c) => app.filename_input.push(c),
                    KeyCode::Backspace => { app.filename_input.pop(); },
                    KeyCode::Esc => { app.state = AppState::Finished; app.filename_input.clear(); app.save_status_message = None; },
                    KeyCode::Enter => {
                        let script = app.generate_commands(app.reboot_requested);
                        match fs::write(&app.filename_input, script) {
                            Ok(_) => app.save_status_message = Some(format!("Saved to {}", app.filename_input)),
                            Err(e) => app.save_status_message = Some(format!("Error: {}", e)),
                        }
                        app.state = AppState::Finished;
                        app.filename_input.clear();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    match app.state {
        AppState::Finished | AppState::Saving => {
            draw_finished_screen(f, app);
            if let AppState::Saving = app.state {
                draw_saving_popup(f, &app.filename_input);
            }
        },
        AppState::Running => {
            draw_main_ui(f, app);
        }
    }
}

fn draw_main_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0), // Main content area
            Constraint::Percentage(40), // Script preview
            Constraint::Length(3), // Footer
        ].as_ref())
        .split(f.size());

    let path_str = {
        app.nav_path.iter().map(|node_rc| {
            let node = node_rc.borrow();
            match &*node {
                MenuNode::Menu { name, .. } => name.clone(),
                MenuNode::Item { name, .. } => name.clone(),
            }
        }).collect::<Vec<_>>().join(" > ")
    };

    let title_text = format!("Enterprise Linux TUI (Detected: {:?})", app.os_distro);
    let title = Paragraph::new(title_text).style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let main_chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let visible_nodes = get_visible_nodes(&app.nav_path);
    let menu_items: Vec<ListItem> = visible_nodes.iter().map(|(text, _)| ListItem::new(text.clone())).collect();

    if !visible_nodes.is_empty() {
        app.selected_index = app.selected_index.min(visible_nodes.len() - 1);
    } else {
        app.selected_index = 0;
    }

    let menu_block = Block::default().title(path_str).borders(Borders::ALL).style(Style::default().fg(Color::Yellow));
    let list = List::new(menu_items)
        .block(menu_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::DarkGray))
        .highlight_symbol(">> ");
    
    let mut list_state = ratatui::widgets::ListState::default();
    if !visible_nodes.is_empty() {
        list_state.select(Some(app.selected_index));
    }
    f.render_stateful_widget(list, main_chunks[0], &mut list_state);

    let selected_items: Vec<ListItem> = app.get_selected_items().iter().map(|s| ListItem::new(s.clone())).collect();
    let selected_list = List::new(selected_items).block(Block::default().borders(Borders::ALL).title("Selected Components"));
    f.render_widget(selected_list, main_chunks[1]);

    let script_content = app.generate_commands(false);
    let script_preview = Paragraph::new(script_content)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Generated Script Preview"));
    f.render_widget(script_preview, chunks[2]);

    let footer_text = "Navigate [←→↑↓] | Select [Enter] | [i] Generate Script | [q] Quit";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}

/// Generates the list of visible nodes with tree-style formatting.
fn get_visible_nodes(nav_path: &[Rc<RefCell<MenuNode>>]) -> Vec<(String, Rc<RefCell<MenuNode>>)> {
    let mut items = Vec::new();
    let current_menu = nav_path.last().unwrap();

    // This recursive helper function builds the tree structure.
    fn build_tree_display(
        items: &mut Vec<(String, Rc<RefCell<MenuNode>>)>,
        node: &Rc<RefCell<MenuNode>>,
        prefix: &str,
        is_last: bool,
    ) {
        let node_borrow = node.borrow();
        let connector = if is_last { "└─" } else { "├─" };
        let line = format!("{}{}", prefix, connector);

        match &*node_borrow {
            MenuNode::Menu { name, children } => {
                items.push((format!("{} {} >", line, name), node.clone()));
                
                let new_prefix = if is_last {
                    format!("{}   ", prefix)
                } else {
                    format!("{}│  ", prefix)
                };
                
                let num_children = children.len();
                for (i, child) in children.iter().enumerate() {
                    build_tree_display(items, child, &new_prefix, i == num_children - 1);
                }
            }
            MenuNode::Item { name, selected, .. } => {
                let prefix_icon = if *selected { "[x]" } else { "[ ]" };
                items.push((format!("{} {} {}", line, prefix_icon, name), node.clone()));
            }
        }
    }

    if let MenuNode::Menu { children, .. } = &*current_menu.borrow() {
        // If we are at the root, render the full tree recursively.
        if nav_path.len() == 1 {
            let num_children = children.len();
            for (i, child) in children.iter().enumerate() {
                build_tree_display(&mut items, child, "", i == num_children - 1);
            }
        } else {
            // If we are in a submenu, render a simple list but still use tree connectors.
            let num_children = children.len();
            for (i, child) in children.iter().enumerate() {
                let node_borrow = child.borrow();
                let connector = if i == num_children - 1 { "└─" } else { "├─" };
                match &*node_borrow {
                    MenuNode::Menu { name, .. } => {
                        items.push((format!("{} {} >", connector, name), child.clone()));
                    }
                    MenuNode::Item { name, selected, .. } => {
                        let prefix_icon = if *selected { "[x]" } else { "[ ]" };
                        items.push((format!("{} {} {}", connector, prefix_icon, name), child.clone()));
                    }
                }
            }
        }
    }
    items
}


fn draw_finished_screen(f: &mut Frame, app: &mut App) {
    // FIX: Changed Constraint.Length to Constraint::Length
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref()).split(f.size());
    let script_content = app.generate_commands(app.reboot_requested);
    let title = if app.reboot_requested { "Installation Script (with Reboot)" } else { "Installation Script" };
    let paragraph = Paragraph::new(script_content).wrap(Wrap { trim: true })
        .block(Block::default().title(title).borders(Borders::ALL));
    f.render_widget(paragraph, chunks[0]);

    if let Some(msg) = &app.save_status_message {
        let msg_p = Paragraph::new(msg.as_str()).style(Style::default().fg(Color::Yellow));
        let area = centered_rect(50, 10, f.size());
        f.render_widget(Clear, area);
        f.render_widget(msg_p.block(Block::default().borders(Borders::ALL).title("Status")), area);
        if app.filename_input.is_empty() { 
             app.save_status_message = None;
        }
    }

    let footer_text = "Review Script | [s] Save to File | [r] Run Directly | [q] Quit | [Esc/Backspace] Go Back";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[1]);
}

fn draw_saving_popup(f: &mut Frame, input: &str) {
    let area = centered_rect(60, 20, f.size());
    let block = Block::default().title("Save Script").borders(Borders::ALL);
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let popup_chunks = Layout::default().direction(Direction::Vertical).margin(2)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Min(1)].as_ref()).split(area);
    
    let p1 = Paragraph::new("Enter filename (press Enter to save, Esc to cancel):");
    let p2 = Paragraph::new(input).block(Block::default().borders(Borders::ALL));
    f.render_widget(p1, popup_chunks[0]);
    f.render_widget(p2, popup_chunks[1]);
}

/// Helper function to create a centered rectangle for popups
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Percentage((100 - percent_y) / 2), Constraint::Percentage(percent_y), Constraint::Percentage((100 - percent_y) / 2)].as_ref())
        .split(r);
    Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage((100 - percent_x) / 2), Constraint::Percentage(percent_x), Constraint::Percentage((100 - percent_x) / 2)].as_ref())
        .split(popup_layout[1])[1]
}
