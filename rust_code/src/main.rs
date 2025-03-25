use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

fn main() {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw).expect("Failed to draw frame");
        if matches!(event::read().expect("Failed to read event"), Event::Key(_)) {
        break;
        }
    }
    ratatui::restore();
}


fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello world!");
    frame.render_widget(text, frame.area());
}

