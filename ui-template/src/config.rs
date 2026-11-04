// ABOUTME: UI configuration state for the sandbox
// ABOUTME: Provides toggleable options for experimenting with UI elements

/// UI elements that can be toggled
#[derive(Debug, Clone, Copy)]
pub enum UiElement {
    SearchBar,
    StatusBar,
    ResultsPane,
    PreviewPane,
    ScopeWidget,
}

/// Theme mode override
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Auto,
    Dark,
    Light,
}

impl ThemeMode {
    pub fn cycle(&self) -> Self {
        match self {
            ThemeMode::Auto => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Auto,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            ThemeMode::Auto => "auto",
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        }
    }
}

/// Configuration state for UI rendering
#[derive(Debug, Clone)]
pub struct UiConfig {
    // Layout toggles
    pub show_search_bar: bool,
    pub show_status_bar: bool,
    pub show_results_pane: bool,
    pub show_preview_pane: bool,
    pub show_scope_widget: bool,

    // Sizing (percentages)
    pub results_width_percent: u16,

    // Theme
    pub theme_mode: ThemeMode,

    // Help overlay
    pub show_help: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_search_bar: true,
            show_status_bar: true,
            show_results_pane: true,
            show_preview_pane: true,
            show_scope_widget: true,
            results_width_percent: 40,
            theme_mode: ThemeMode::Auto,
            show_help: false,
        }
    }
}

impl UiConfig {
    /// Toggle a specific UI element
    pub fn toggle(&mut self, element: UiElement) {
        match element {
            UiElement::SearchBar => self.show_search_bar = !self.show_search_bar,
            UiElement::StatusBar => self.show_status_bar = !self.show_status_bar,
            UiElement::ResultsPane => self.show_results_pane = !self.show_results_pane,
            UiElement::PreviewPane => self.show_preview_pane = !self.show_preview_pane,
            UiElement::ScopeWidget => self.show_scope_widget = !self.show_scope_widget,
        }
    }

    /// Cycle through theme modes
    pub fn cycle_theme(&mut self) {
        self.theme_mode = self.theme_mode.cycle();
    }

    /// Adjust the results pane width (-5 to +5 percent)
    pub fn adjust_split(&mut self, delta: i16) {
        let new_width = (self.results_width_percent as i16 + delta).clamp(20, 80) as u16;
        self.results_width_percent = new_width;
    }

    /// Reset to default configuration
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Toggle help overlay
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Get preview width (complement of results width)
    pub fn preview_width_percent(&self) -> u16 {
        100 - self.results_width_percent
    }
}
