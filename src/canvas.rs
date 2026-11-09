use crate::graph::{CommitNode, GitRef, RefType};
use iced::widget::canvas::{Frame, Geometry, Path, Program, Stroke, Text};
use iced::mouse::Cursor;
use iced::{Color, Point, Rectangle, Theme};
use iced::Renderer;
use std::collections::HashMap;

pub struct GitGraphCanvas {
    commits: Vec<CommitNode>,
    refs: Vec<GitRef>,
    index_map: HashMap<String, usize>,
}

impl GitGraphCanvas {
    pub fn new(commits: &[CommitNode], refs: &[GitRef]) -> Self {
        let index_map: HashMap<String, usize> = commits
            .iter()
            .enumerate()
            .map(|(i, c)| (c.hash.clone(), i))
            .collect();

        Self {
            commits: commits.to_vec(),
            refs: refs.to_vec(),
            index_map,
        }
    }
}

impl Program<crate::message::Message> for GitGraphCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let size = bounds.size();
        let mut frame = Frame::new(renderer, size);

        let mut refs_by_commit: HashMap<String, Vec<GitRef>> = HashMap::new();
        for r in &self.refs {
            refs_by_commit
                .entry(r.target.clone())
                .or_default()
                .push(r.clone());
        }

        let node_radius = 5.0;
        let line_width = 2.0;

        let lane_colors: Vec<Color> = vec![
            Color::from_rgb(0.35, 0.65, 1.0),
            Color::from_rgb(1.0, 0.55, 0.25),
            Color::from_rgb(0.45, 0.85, 0.45),
            Color::from_rgb(0.85, 0.35, 0.85),
            Color::from_rgb(1.0, 0.85, 0.25),
            Color::from_rgb(0.95, 0.35, 0.35),
        ];

        for commit in &self.commits {
            let child_point = Point::new(commit.x, commit.y);

            for parent_hash in &commit.parents {
                if let Some(&pi) = self.index_map.get(parent_hash) {
                    let parent = &self.commits[pi];
                    let parent_point = Point::new(parent.x, parent.y);
                    let color = lane_colors[commit.lane % lane_colors.len()];

                    if (child_point.x - parent_point.x).abs() < 0.1 {
                        let line = Path::line(child_point, parent_point);
                        frame.stroke(
                            &line,
                            Stroke {
                                style: iced_graphics::geometry::Style::Solid(color),
                                width: line_width,
                                ..Stroke::default()
                            },
                        );
                    } else {
                        let mid_y = (child_point.y + parent_point.y) / 2.0;
                        let path = Path::new(|builder| {
                            builder.move_to(child_point);
                            builder.quadratic_curve_to(
                                Point::new((child_point.x + parent_point.x) / 2.0, mid_y),
                                parent_point,
                            );
                        });
                        frame.stroke(
                            &path,
                            Stroke {
                                style: iced_graphics::geometry::Style::Solid(color),
                                width: line_width,
                                ..Stroke::default()
                            },
                        );
                    }
                }
            }

            let point = Point::new(commit.x, commit.y);
            let color = lane_colors[commit.lane % lane_colors.len()];
            let radius = if commit.parents.len() > 1 {
                node_radius + 2.0
            } else {
                node_radius
            };

            let circle = Path::circle(point, radius);
            frame.fill(&circle, color);
            frame.stroke(
                &circle,
                Stroke {
                    style: iced_graphics::geometry::Style::Solid(Color::WHITE),
                    width: 1.5,
                    ..Stroke::default()
                },
            );

            // Draw branch/tag labels to the right of the node.
            if let Some(commit_refs) = refs_by_commit.get(&commit.hash) {
                let mut x = point.x + radius + 8.0;
                for r in commit_refs {
                    let (fill_color, text_color) = match r.ref_type {
                        RefType::Branch => (
                            Color::from_rgb(0.2, 0.45, 0.85),
                            Color::WHITE,
                        ),
                        RefType::Tag => (
                            Color::from_rgb(0.85, 0.55, 0.15),
                            Color::WHITE,
                        ),
                        RefType::Other => (Color::from_rgb(0.5, 0.5, 0.5), Color::WHITE),
                    };

                    let label = r.short_name.clone();
                    let width = (label.len() as f32) * 7.0 + 12.0;
                    let height = 14.0;
                    let rect = Path::rounded_rectangle(
                        Point::new(x, point.y - height / 2.0),
                        iced::Size::new(width, height),
                        4.0.into(),
                    );
                    frame.fill(&rect, fill_color);

                    frame.fill_text(Text {
                        content: label,
                        position: Point::new(x + 6.0, point.y),
                        color: text_color,
                        size: 11.0.into(),
                        vertical_alignment: iced::alignment::Vertical::Center,
                        ..Text::default()
                    });

                    x += width + 4.0;
                }
            }
        }

        vec![frame.into_geometry()]
    }
}
