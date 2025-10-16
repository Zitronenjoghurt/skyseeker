use crate::ui::composites::toolbar::ToolBar;
use crate::ui::composites::AppComposite;
use crate::ui::windows::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::EguiContexts;

pub fn render_ui(mut contexts: EguiContexts, mut window_manager: ResMut<WindowManager>) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    ToolBar::new(&mut window_manager).show(ctx);
    window_manager.render(ctx);
}
