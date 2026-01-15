use advent_of_code_2023::*;
use std::{
    io,
    time::{Duration, Instant},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Padding, Paragraph},
};

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    println!("Solving all 25 days...");
    let solutions = solutions();
    terminal.clear()?;
    loop {
        terminal.draw(|frame| draw(frame, &solutions))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}

fn draw(frame: &mut Frame, solutions: &[(usize, usize, Duration)]) {
    let main_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let block_layout = Layout::vertical([Constraint::Length(20); 5]);
    let [title_area, main_area, footer_area] = main_layout.areas(frame.area());
    let areas: Vec<Vec<Rect>> = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(area)
            .to_vec()
        })
        .collect();
    frame.render_widget(
        Paragraph::new("Advent of Code 2023")
            .alignment(ratatui::layout::Alignment::Center)
            .style(
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        title_area,
    );
    frame.render_widget(
        Paragraph::new("https://github.com/wjholden/Advent-of-Code-2023")
            .alignment(ratatui::layout::Alignment::Center),
        footer_area,
    );
    for row in 0..5 {
        for col in 0..5 {
            let day = 1 + row * 5 + col;
            let message = match solutions.get(day - 1) {
                Some((part1, 0, duration)) => {
                    format!("Part 1:  {part1}\nPart 2:  Merry Christmas!\n{duration:?}")
                }
                Some((part1, part2, duration)) => {
                    format!("Part 1:  {part1}\nPart 2:  {part2}\nRuntime: {duration:?}")
                }
                None => "Coming soon!".to_owned(),
            };
            frame.render_widget(
                Paragraph::new(message).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .padding(Padding::new(1, 1, 1, 1))
                        .title(format!("Day {day}")),
                ),
                areas[row][col],
            );
        }
    }
}

fn solutions() -> Vec<(usize, usize, std::time::Duration)> {
    vec![
        {
            let start = Instant::now();
            let puzzle = day01::PUZZLE;
            let part1 = day01::solve(puzzle, Part::One);
            let part2 = day01::solve(puzzle, Part::Two);
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let puzzle = day02::PUZZLE.trim();
            let games = day02::parse_games(puzzle).unwrap();
            let part1 = day02::part1(&games).unwrap();
            let part2 = day02::part2(&games).unwrap();
            (part1 as usize, part2 as usize, start.elapsed())
        },
        {
            let start = Instant::now();
            let (part1, part2) = day03::solve(day03::PUZZLE).unwrap();
            (part1 as usize, part2 as usize, start.elapsed())
        },
        {
            let start = Instant::now();
            let part1 = day04::part1(day04::PUZZLE);
            let part2 = day04::part2(day04::PUZZLE);
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let puzzle = day05::PUZZLE.trim();
            let (seeds, layers) = day05::parse(puzzle);
            let part1 = day05::part1(&seeds, &layers);
            let part2 = day05::part2(&seeds, &layers).unwrap();
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let part1 = day06::quadratic(day06::PUZZLE);
            let part2 = day06::parse2(day06::PUZZLE).unwrap().quadratic();
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let part1 = day07::solve(day07::PUZZLE, Part::One);
            let part2 = day07::solve(day07::PUZZLE, Part::Two);
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let n = day08::Network::new(day08::PUZZLE);
            let part1 = n.zzz("AAA").unwrap();
            let part2 = n.part2_lcm();
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let mut histories = day09::parse(day09::PUZZLE);
            let part1 = histories.iter().map(|v| day09::predict(v)).sum::<isize>();

            for history in &mut histories {
                history.reverse();
            }

            let part2 = histories.iter().map(|v| day09::predict(v)).sum::<isize>();

            (part1 as usize, part2 as usize, start.elapsed())
        },
        {
            let start = Instant::now();
            let pipes = day10::Pipes::new(day10::PUZZLE);
            let (part1, part2) = pipes.solve();
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let image = day11::Image::new(day11::PUZZLE);
            let part1 = image.predict(2);
            let part2 = image.predict(1_000_000);
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let mut springs = day12::Springs::new(day12::PUZZLE);
            let part1 = springs.total_arrangements();
            springs.unfold();
            let part2 = springs.total_arrangements();
            (part1, part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day13::Puzzle::new(day13::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day14::Puzzle::new(day14::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day15::Puzzle::new(day15::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day16::Puzzle::new(day16::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day17::Puzzle::new(day17::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day18::Puzzle::new(day18::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day19::Puzzle::new(day19::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day20::Puzzle::new(day20::PUZZLE).solve();
            (d.part1.unwrap(), d.part2.unwrap(), start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day21::Puzzle::new(day21::PUZZLE).solve();
            (d.part1.unwrap(), d.part2.unwrap(), start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day22::Puzzle::new(day22::PUZZLE).solve();
            (d.part1.unwrap(), d.part2.unwrap(), start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day23::Puzzle::new(day23::PUZZLE).solve();
            (d.part1.unwrap(), d.part2.unwrap(), start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day24::Puzzle::new(day24::PUZZLE).solve();
            (
                d.part1.unwrap(),
                d.part2.unwrap() as usize + 1, /* solution has a known off-by-one error */
                start.elapsed(),
            )
        },
        {
            let start = Instant::now();
            let d = day25::Puzzle::new(day25::PUZZLE).solve();
            (d.part1.unwrap(), 0, start.elapsed())
        },
    ]
}
