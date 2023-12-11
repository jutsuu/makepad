use proc_macro::{TokenStream};

mod derive_widget;
use crate::derive_widget::*;

#[proc_macro_derive(DefaultNone)]
pub fn derive_widget_action(input: TokenStream) -> TokenStream {
    derive_default_none_impl(input)
}
/*
#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    derive_widget_impl(input)
}*/

#[proc_macro_derive(WidgetRedraw, attributes(
    walk,
    deref,
    redraw,
    walk_redraw
))]
pub fn derive_widget_redraw(input: TokenStream) -> TokenStream {
    derive_widget_redraw_impl(input)
}

#[proc_macro_derive(WidgetRef)]
pub fn derive_widget_ref(input: TokenStream) -> TokenStream {
    derive_widget_ref_impl(input)
}

#[proc_macro_derive(LiveRegisterWidget)]
pub fn derive_widget_register(input: TokenStream) -> TokenStream {
    derive_widget_register_impl(input)
}

#[proc_macro_derive(WidgetSet)]
pub fn derive_widget_set(input: TokenStream) -> TokenStream {
    derive_widget_set_impl(input)
}
