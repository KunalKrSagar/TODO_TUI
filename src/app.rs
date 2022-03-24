use tui::widgets::ListState;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToDoItem {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Priority {
    critical(String), 
    moderate(String), 
    low(String)
}
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::critical(s) => write!(f,"{}", s),
            Priority::moderate(s) => write!(f,"{}", s),
            Priority::low(s) => write!(f,"{}", s)
        }
    }
}
fn create_data<'a>() -> Result<Vec<ToDoItem>> {
    let data = r#"
        {
            "id": 1,
            "title": "title for Task1",
            "description": "description for Task 1", 
            "priority": {"critical": "critical"}
        }"#;
    
    let t1: ToDoItem = serde_json::from_str(data)?;
    let data = r#"
        {
            "id": 2,
            "title":  "Title for Task2",
            "description": "Description for Task 2 ",
            "priority": {"moderate": "moderate"}
        }"#;
    let t2: ToDoItem = serde_json::from_str(data)?;
    let fin = vec![t1, t2]; 
    Ok(fin)
}


pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub enhanced_graphics: bool,
    pub todo_list: StatefulList<ToDoItem>,
    /// Current value of the input box
    pub input: String,
    /// History of recorded messages
    pub messages: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["TO-DO", "In-progress", "Done","Add-task",]),
            enhanced_graphics,
            todo_list: match create_data() {
                Ok(t) => StatefulList::with_items(t),
                Err(error) => panic!("Problem reading JSON {:?}", error),
            },
            input: String::new(),
            messages: Vec::new(),
        }
    }
    pub fn on_up(&mut self) {
        self.todo_list.previous();
    }

    pub fn on_down(&mut self) {
        self.todo_list.next();
    }
    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            '\n'=> {
                self.messages.push(self.input.drain(..).collect());
            }
            _  => {
                if self.tabs.index == 3 {
                    self.input.push(c);
                }
            }
        }

    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}