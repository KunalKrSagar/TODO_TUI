
use crate::app::App;
use crate::app::ToDoItem;
use crate::app::Priority;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{
        Block, Borders, Cell, List, ListItem,
        Paragraph, Row, Table, Tabs, Wrap,
    },
    Frame,
};
use unicode_width::UnicodeWidthStr;
const TODO_ITEM_ATTRIBUTES : [&str; 4] = ["ID: ", "Title: ", "Description: ", "Priority: "];

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        2 => draw_third_tab(f, app, chunks[1]),
        3 => draw_fourth_tab(f, app, chunks[1]),
        _ => {}
    };
}
fn draw_fourth_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
     let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    let (msg, style) ={(
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to Clear, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record"),
            ],
            Style::default(),
        )
    };

    let mut mess_len: usize = app.messages.len();
    if mess_len == 4 {
        app.todo_list.items.push(
            ToDoItem {
                id: app.messages[0].parse().unwrap(),
                title: app.messages[1].clone(), 
                description: app.messages[2].clone(), 
                priority: match app.messages[3].as_str(){
                    "critical" => {Priority::critical(app.messages[3].clone())}, 
                    "moderate" => {Priority::moderate(app.messages[3].clone())}, 
                    "low" => {Priority::low(app.messages[3].clone())}, 
                    _ => Priority::low("".to_string())
                }
            }
        );
        app.messages.clear(); 
        mess_len = 0;
        let (msg, style) ={(
                vec![
                Span::styled("TODO ITEM ADDED", Style::default().fg(Color::Green)),
                ],
                Style::default(),
            )
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[3]);
    }

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[1]);

    let fin_str = TODO_ITEM_ATTRIBUTES[mess_len].to_string() + app.input.as_ref();
    let fin_str_len = TODO_ITEM_ATTRIBUTES[mess_len].to_string().width();

    let input = Paragraph::new(fin_str.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[2]);
    

    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    f.set_cursor(
        // Put cursor past the end of the input text
        fin_str_len as u16 + chunks[2].x + app.input.width() as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[2].y + 1,
    );

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}{}", TODO_ITEM_ATTRIBUTES[i], m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Creating TODO Item"));
    f.render_widget(messages, chunks[0]);
}
fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    render_todo_list(f, app, chunks[0]);
    draw_text(f, chunks[1]);
}
fn render_todo_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect) 
where
    B: Backend
{
    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
        .as_ref(),
    )
    .split(area);

    
    // Draw TODO    
    let tasks: Vec<ListItem> = app.todo_list.items
    .iter()
    .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.title.clone()))]))
    .collect();

    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("todo-list"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(tasks, chunks[0], &mut app.todo_list.state);
    
    let current_todo = match app.todo_list.state.selected() {
        Some(i) => i,
        None => 0,
    };
    if app.todo_list.items.len() == 0 {
        return;
    }
    let current_todo = &app.todo_list.items[current_todo];
    let id = &current_todo.id.to_string();
    let prio = &current_todo.priority.to_string();
    let text = vec![Spans::from(vec![
                Span::from("Id: "), 
                Span::styled(id, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                ], 
            ),
            Spans::from(""),
            Spans::from(vec![
                Span::from("Title: "), 
                Span::styled(&current_todo.title, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            ]),

            Spans::from(""),
            Spans::from(vec![
                Span::from("Description: "), 
                Span::styled(&current_todo.description, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            ]),

            Spans::from(""),
            Spans::from(vec![
                Span::from("Priority: "), 
                Span::styled(prio, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            ])  
    ];
    
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Todo-Details",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);

    // let task_details = List::new( [ListItem::new(cuurent_todo)])
    //     .block(Block::default().borders(Borders::ALL).title("todo-detail"));
    // f.render_stateful_widget(task_details, chunks[1], &mut app.todo_list.state);

}
fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from(""),
        Spans::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        Spans::from(vec![
            Span::raw("Oh and if you didn't "),
            Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" you can "),
            Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
            Span::raw(" your "),
            Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::raw(".")
        ]),
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "LOGS",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}


fn draw_third_tab<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);
    let colors = [
        Color::Reset,
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Gray,
        Color::DarkGray,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
        Color::White,
    ];
    let items: Vec<Row> = colors
        .iter()
        .map(|c| {
            let cells = vec![
                Cell::from(Span::raw(format!("{:?}: ", c))),
                Cell::from(Span::styled("Foreground", Style::default().fg(*c))),
                Cell::from(Span::styled("Background", Style::default().bg(*c))),
            ];
            Row::new(cells)
        })
        .collect();
    let table = Table::new(items)
        .block(Block::default().title("Colors").borders(Borders::ALL))
        .widths(&[
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);
    f.render_widget(table, chunks[0]);
}
