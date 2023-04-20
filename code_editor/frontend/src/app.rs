use {
    crate::{code_editor, code_editor::CodeEditor},
    std::{cell::RefCell, rc::Rc},
    makepad_code_editor_core::{Document, Session},
    makepad_widgets::*,
};

live_design! {
    import makepad_widgets::desktop_window::DesktopWindow;
    
    App = {{App}} {
        ui: <DesktopWindow> {
            frame: {
                body = {
                    user_draw:true
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
#[live_design_with {
    makepad_widgets::live_design(cx);
    code_editor::live_design(cx);
}]
pub struct App {
    ui: WidgetRef,
    editor: CodeEditor,
    #[rust(AppState::new(cx))]
    app_state: AppState,
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            let mut cx = Cx2d::new(cx, event);
            if self.ui.draw_widget_continue(&mut cx).is_not_done() {
                self.editor.draw(&mut cx, &*self.app_state.session.borrow());
                self.ui.draw_widget(&mut cx);
            }
            return;
        }
        self.ui.handle_widget_event(cx, event);
        self.editor.handle_event(cx, event);
    }
}

struct AppState {
    session: Rc<RefCell<Session>>,
}

impl AppState {
    pub fn new(cx: &mut Cx) -> Self {
        use {makepad_code_editor_core::Text, makepad_futures::channel::oneshot, std::{thread, time::Duration}};

        let (sender, receiver) = oneshot::channel();
        thread::spawn(move|| {
            thread::sleep(Duration::from_secs(3));
            sender.send(Text::from([
                "Hello, world!".into()
            ])).unwrap();
        });
        let document = Document::new(cx.spawner(), {
            let cx = cx.get_ref().0;
            async move {
                let text = receiver.await.unwrap();
                cx.borrow_mut().redraw_all();
                text
            }
        });
        AppState {
            session: Session::new(document),
        }
    }
}

app_main!(App);