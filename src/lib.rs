use chrono::{Days, NaiveDate, Weekday};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

const BLOCK_PATTERN: [&str; 5] = [" ", "░", "▒", "▓", "█"];

pub type ActivityCellData = (NaiveDate, u16);

pub struct ActivityGraph {
    data: Vec<ActivityCellData>,
    last_day: NaiveDate,
    first_weekday: Weekday,
    weeks: u16,
}

impl ActivityGraph {
    pub fn new(data: Vec<ActivityCellData>, last_day: NaiveDate) -> Self {
        Self {
            data,
            last_day,
            first_weekday: Weekday::Sun,
            weeks: 14,
        }
    }

    pub fn with_weeks(mut self, weeks: u16) -> Self {
        self.weeks = weeks;
        self
    }

    pub fn with_first_weekday(mut self, first_weekday: Weekday) -> Self {
        self.first_weekday = first_weekday;
        self
    }
}

impl Widget for ActivityGraph {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let max = self
            .data
            .iter()
            .map(|item| item.1)
            .max_by_key(|item| *item)
            .unwrap_or(1);

        let weeks_iterator = (self.last_day - Days::new(7 * (self.weeks as u64 - 1)))
            .week(self.first_weekday)
            .first_day()
            .iter_weeks();

        for (column, week_start) in area.columns().take(self.weeks as usize).zip(weeks_iterator) {
            for (row, day) in column.rows().take(7).zip(week_start.iter_days().take(7)) {
                if day > self.last_day {
                    break;
                }
                let value = self
                    .data
                    .iter()
                    .find(|item| item.0 == day)
                    .map(|item| item.1)
                    .unwrap_or(0);
                match value {
                    0 => buf[row].set_symbol(BLOCK_PATTERN[0]),
                    1.. => {
                        let normalized_value =
                            (((value as f32 / max as f32) * 4f32) as usize).max(1);
                        buf[row].set_symbol(BLOCK_PATTERN[normalized_value])
                    }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn data() -> Vec<ActivityCellData> {
        vec![
            (
                NaiveDate::parse_from_str("2024-12-28", "%Y-%m-%d").unwrap(),
                4,
            ),
            (
                NaiveDate::parse_from_str("2025-01-01", "%Y-%m-%d").unwrap(),
                1,
            ),
            (
                NaiveDate::parse_from_str("2025-01-02", "%Y-%m-%d").unwrap(),
                4,
            ),
            (
                NaiveDate::parse_from_str("2025-01-03", "%Y-%m-%d").unwrap(),
                2,
            ),
            (
                NaiveDate::parse_from_str("2025-01-04", "%Y-%m-%d").unwrap(),
                3,
            ),
            (
                NaiveDate::parse_from_str("2025-01-05", "%Y-%m-%d").unwrap(),
                12,
            ),
            (
                NaiveDate::parse_from_str("2025-01-06", "%Y-%m-%d").unwrap(),
                5,
            ),
            (
                NaiveDate::parse_from_str("2025-01-07", "%Y-%m-%d").unwrap(),
                4,
            ),
            (
                NaiveDate::parse_from_str("2025-01-09", "%Y-%m-%d").unwrap(),
                2,
            ),
            (
                NaiveDate::parse_from_str("2025-01-10", "%Y-%m-%d").unwrap(),
                18,
            ),
            (
                NaiveDate::parse_from_str("2025-01-11", "%Y-%m-%d").unwrap(),
                26,
            ),
            (
                NaiveDate::parse_from_str("2025-01-15", "%Y-%m-%d").unwrap(),
                24,
            ),
            (
                NaiveDate::parse_from_str("2025-01-16", "%Y-%m-%d").unwrap(),
                15,
            ),
            (
                NaiveDate::parse_from_str("2025-01-17", "%Y-%m-%d").unwrap(),
                6,
            ),
            (
                NaiveDate::parse_from_str("2025-01-19", "%Y-%m-%d").unwrap(),
                2,
            ),
        ]
    }

    #[rstest]
    fn test_render(data: Vec<ActivityCellData>) {
        let activity_graph = ActivityGraph::new(
            data,
            NaiveDate::parse_from_str("2025-01-19", "%Y-%m-%d").unwrap(),
        )
        .with_weeks(4);

        let mut buf = Buffer::empty(Rect::new(0, 0, 8, 8));
        activity_graph.render(Rect::new(0, 0, 8, 8), &mut buf);

        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                " ░ ░    ",
                " ░      ",
                " ░      ",
                "░ ▓     ",
                "░░▒     ",
                "░▒░     ",
                "░█      ",
                "        ",
            ])
        );
    }

    #[rstest]
    fn test_render_with_first_weekday(data: Vec<ActivityCellData>) {
        let activity_graph = ActivityGraph::new(
            data,
            NaiveDate::parse_from_str("2025-01-19", "%Y-%m-%d").unwrap(),
        )
        .with_weeks(4)
        .with_first_weekday(Weekday::Mon);

        let mut buf = Buffer::empty(Rect::new(0, 0, 8, 8));
        activity_graph.render(Rect::new(0, 0, 8, 8), &mut buf);

        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "  ░     ",
                "  ░     ",
                " ░ ▓    ",
                " ░░▒    ",
                " ░▒░    ",
                "░░█     ",
                " ░ ░    ",
                "        ",
            ])
        );
    }
}
