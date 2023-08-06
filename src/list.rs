use ratatui::widgets::ListState;

pub struct List<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> List<T> {
    pub fn new(items: Vec<T>) -> List<T> {
        let mut state = ListState::default();

        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { state, items }
    }

    pub fn next(&mut self) {
        if let Some(index) = self.state.selected() {
            self.state.select(Some((index + 1) % self.items.len()));
        };
    }

    pub fn previous(&mut self) {
        if let Some(index) = self.state.selected() {
            let index = if index == 0 {
                self.items.len() - 1
            } else {
                index - 1
            };
            self.state.select(Some(index));
        };
    }

    pub fn selected(&mut self) -> Option<&mut T> {
        self.state
            .selected()
            .map(move |index| &mut self.items[index])
    }
}
