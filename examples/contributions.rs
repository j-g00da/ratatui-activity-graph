use chrono::NaiveDate;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Borders};
use ratatui_activity_graph::{ActivityCellData, ActivityGraph};
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ContributionDay {
    date: NaiveDate,
    count: u16,
    level: u16,
}

#[derive(Debug, Deserialize)]
struct Contributions {
    contributions: Vec<ContributionDay>,
}

fn fetch_contributions(
    username: &str,
) -> Result<Vec<ActivityCellData>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://github-contributions-api.jogruber.de/v4/{}",
        username
    );
    let response = get(&url)?.json::<Contributions>()?;

    let mut data = Vec::new();
    for day in response.contributions {
        data.push((day.date, day.level));
    }

    Ok(data)
}

fn main() {
    let username = "j-g00da";
    let mut terminal = ratatui::init();
    match fetch_contributions(username) {
        Ok(data) => {
            terminal
                .draw(|frame| {
                    let contrib_graph =
                        ActivityGraph::new(data, chrono::Local::now().date_naive()).with_weeks(40);
                    let block = Block::new()
                        .borders(Borders::all())
                        .title("j-g00da's GitHub Contributions");
                    let block_area = Rect::new(0, 0, 42, 9);
                    let inner_area = block.inner(block_area);
                    frame.render_widget(block.bg(Color::Black), block_area);
                    frame.render_widget(contrib_graph, inner_area);
                })
                .unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
