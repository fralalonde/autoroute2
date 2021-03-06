use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, Paragraph, Row, Sparkline,
        Table, Tabs, Text,
    },
    Frame,
};

use crate::tui::model::Model;

pub fn draw_root<B: Backend>(f: &mut Frame<B>, app: &mut Model) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let tabs = Tabs::default()
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .titles(&app.tabs.titles)
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, model: &mut Model, area: Rect)
where
    B: Backend,
{
    // let chunks = Layout::default()
    //     .constraints([Constraint::Length(7), Constraint::Min(7), Constraint::Length(7)].as_ref())
    //     .split(area);
    // draw_gauges(f, model, chunks[0]);
    draw_charts(f, model, area);
    // draw_text(f, chunks[2]);
}

fn draw_charts<B: Backend>(f: &mut Frame<B>, model: &mut Model, area: Rect) {
    let constraints = if
    /*model.show_chart*/
    true {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let chunks =
        Layout::default().constraints(constraints).direction(Direction::Horizontal).split(area);

    // Draw tasks
    let tasks = model.ports.items.iter().map(|i| Text::raw(i));
    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(tasks, chunks[0], &mut model.ports.state);

    //
    // {
    //     let chunks = Layout::default()
    //         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //         .split(chunks[0]);
    //     {
    //         let chunks = Layout::default()
    //             .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //             .direction(Direction::Horizontal)
    //             .split(chunks[0]);
    //
    //         // Draw tasks
    //         let tasks = app.tasks.items.iter().map(|i| Text::raw(*i));
    //         let tasks = List::new(tasks)
    //             .block(Block::default().borders(Borders::ALL).title("List"))
    //             .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
    //             .highlight_symbol("> ");
    //         f.render_stateful_widget(tasks, chunks[0], &mut app.tasks.state);
    //
    //         // Draw logs
    //         let info_style = Style::default().fg(Color::White);
    //         let warning_style = Style::default().fg(Color::Yellow);
    //         let error_style = Style::default().fg(Color::Magenta);
    //         let critical_style = Style::default().fg(Color::Red);
    //         let logs = app.logs.items.iter().map(|&(evt, level)| {
    //             Text::styled(
    //                 format!("{}: {}", level, evt),
    //                 match level {
    //                     "ERROR" => error_style,
    //                     "CRITICAL" => critical_style,
    //                     "WARNING" => warning_style,
    //                     _ => info_style,
    //                 },
    //             )
    //         });
    //         let logs = List::new(logs).block(Block::default().borders(Borders::ALL).title("List"));
    //         f.render_stateful_widget(logs, chunks[1], &mut app.logs.state);
    //     }
    //
    //     let barchart = BarChart::default()
    //         .block(Block::default().borders(Borders::ALL).title("Bar chart"))
    //         .data(&app.barchart)
    //         .bar_width(3)
    //         .bar_gap(2)
    //         .bar_set(if app.enhanced_graphics {
    //             symbols::bar::NINE_LEVELS
    //         } else {
    //             symbols::bar::THREE_LEVELS
    //         })
    //         .value_style(
    //             Style::default()
    //                 .fg(Color::Black)
    //                 .bg(Color::Green)
    //                 .modifier(Modifier::ITALIC),
    //         )
    //         .label_style(Style::default().fg(Color::Yellow))
    //         .style(Style::default().fg(Color::Green));
    //     f.render_widget(barchart, chunks[1]);
    // }
    // if app.show_chart {
    //     let x_labels = [
    //         format!("{}", app.signals.window[0]),
    //         format!("{}", (app.signals.window[0] + app.signals.window[1]) / 2.0),
    //         format!("{}", app.signals.window[1]),
    //     ];
    //     let datasets = [
    //         Dataset::default()
    //             .name("data2")
    //             .marker(symbols::Marker::Dot)
    //             .style(Style::default().fg(Color::Cyan))
    //             .data(&app.signals.sin1.points),
    //         Dataset::default()
    //             .name("data3")
    //             .marker(if app.enhanced_graphics {
    //                 symbols::Marker::Braille
    //             } else {
    //                 symbols::Marker::Dot
    //             })
    //             .style(Style::default().fg(Color::Yellow))
    //             .data(&app.signals.sin2.points),
    //     ];
    //     let chart = Chart::default()
    //         .block(
    //             Block::default()
    //                 .title("Chart")
    //                 .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
    //                 .borders(Borders::ALL),
    //         )
    //         .x_axis(
    //             Axis::default()
    //                 .title("X Axis")
    //                 .style(Style::default().fg(Color::Gray))
    //                 .labels_style(Style::default().modifier(Modifier::ITALIC))
    //                 .bounds(app.signals.window)
    //                 .labels(&x_labels),
    //         )
    //         .y_axis(
    //             Axis::default()
    //                 .title("Y Axis")
    //                 .style(Style::default().fg(Color::Gray))
    //                 .labels_style(Style::default().modifier(Modifier::ITALIC))
    //                 .bounds([-20.0, 20.0])
    //                 .labels(&["-20", "0", "20"]),
    //         )
    //         .datasets(&datasets);
    //     f.render_widget(chart, chunks[1]);
    // }
}

fn draw_second_tab<B: Backend>(f: &mut Frame<B>, app: &mut Model, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let map = Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.draw(&Map { color: Color::White, resolution: MapResolution::High });
            ctx.layer();
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 30.0,
                width: 10.0,
                height: 10.0,
                color: Color::Yellow,
            });
        })
        .marker(symbols::Marker::Dot)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(map, chunks[1]);
}

// fn draw_gauges<B: Backend>(f: &mut Frame<B>, app: &mut Model, area: Rect) {
//     f.render_widget(Block::default().borders(Borders::ALL).title("Graphs"), area);
//
//     let chunks: Vec<Rect> = Layout::default()
//         .constraints([Constraint::Length(2), Constraint::Length(3)].as_ref())
//         .margin(1)
//         .split(area);
//
//     f.render_widget(
//         Gauge::default()
//             .block(Block::default().title("Gauge:"))
//             .style(
//                 Style::default()
//                     .fg(Color::Magenta)
//                     .bg(Color::Black)
//                     .modifier(Modifier::ITALIC | Modifier::BOLD),
//             )
//             .label(&format!("{:.2}%", 100.0)),
//         chunks[0],
//     );
//
//     f.render_widget(
//         Sparkline::default()
//             .block(Block::default().title("Sparkline:"))
//             .style(Style::default().fg(Color::Green))
//             // .data(&app.sparkline.points)
//             .bar_set(symbols::bar::THREE_LEVELS),
//         chunks[1],
//     );
// }

// fn draw_text<B: Backend>(f: &mut Frame<B>, area: Rect) {
//     let text = [
//         Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFox example: "),
//         Text::styled("under", Style::default().fg(Color::Red)),
//         Text::raw(" "),
//         Text::styled("rainbow", Style::default().fg(Color::Blue)),
//         Text::raw(".\nOh and if you didn't "),
//         Text::styled("notice", Style::default().modifier(Modifier::ITALIC)),
//     ];
//     let block = Block::default()
//         .borders(Borders::ALL)
//         .title("Footer")
//         .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD));
//     let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
//     f.render_widget(paragraph, area);
// }
