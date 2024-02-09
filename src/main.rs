mod app;
mod control_flow;
mod controlled_form;
mod for_example;
mod progress_bar;
mod select;
mod text_area;
mod uncontrolled_form;

use crate::{
    app::DerivedSignalExample, control_flow::ControlFlow, controlled_form::ControlledForm,
    for_example::ForExample, select::Select, text_area::TextArea,
    uncontrolled_form::UncontrolledForm,
};
use leptos::*;

fn main() {
    let a = 1;
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <ControlFlow/>
            <Select/>
            <TextArea/>
            <ControlledForm/>
            <UncontrolledForm/>
            <DerivedSignalExample/>
            <ForExample/>
        }
    })
}
