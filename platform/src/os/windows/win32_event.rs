use {
    crate::{
        window::WindowId,
        event::{
            MouseDownEvent,
            MouseUpEvent,
            MouseMoveEvent,
            ScrollEvent,
            WindowGeomChangeEvent,
            WindowDragQueryEvent,
            WindowCloseRequestedEvent,
            WindowClosedEvent,
            TextInputEvent,
            KeyEvent,
            DragEvent,
            DropEvent,
            TextCopyEvent,
            TextCutEvent,
            TimerEvent,
        },
    }
};

#[derive(Debug)]
pub enum Win32Event {
    AppGotFocus,
    AppLostFocus,
    WindowResizeLoopStart(WindowId),
    WindowResizeLoopStop(WindowId),
    WindowGeomChange(WindowGeomChangeEvent),
    WindowClosed(WindowClosedEvent),
    Paint,
    
    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MouseMove(MouseMoveEvent),
    Scroll(ScrollEvent),
    
    WindowDragQuery(WindowDragQueryEvent),
    WindowCloseRequested(WindowCloseRequestedEvent),
    TextInput(TextInputEvent),
    Drag(DragEvent),
    Drop(DropEvent),
    DragEnd,
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    TextCopy(TextCopyEvent),
    TextCut(TextCutEvent),
    Timer(TimerEvent),
    Signal,
}
