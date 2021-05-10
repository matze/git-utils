use tui::widgets::ListState;

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
        let index = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }

    pub fn previous(&mut self) {
        let index = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }

    pub fn selected(&mut self) -> Option<&mut T> {
        match self.state.selected() {
            Some(index) => Some(&mut self.items[index]),
            None => None,
        }
    }
}
