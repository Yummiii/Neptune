use self::gui_manager::open_block_gui;

mod gui_manager;

pub struct GuiConfigs {
    pub image: Option<String>,
    pub show_cursor: bool
}

pub fn open(cfgs: GuiConfigs) {
    open_block_gui(cfgs)
}