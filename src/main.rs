mod components;
mod pages;

use leptos::*;

use crate::{
    components::{
        control_flow::ControlFlow, controlled_form::ControlledForm, for_example::ForExample,
        select::Select, text_area::TextArea, uncontrolled_form::UncontrolledForm,
    },
    pages::{
        children::ChildrenPage, communication::CommunicationPage,
        derived_signal::DerivedSignalPage, numeric_input::NumericInput, show::ShowPage,
        working_with_signals::WorkingWithSignalsPage,
    },
};

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <WorkingWithSignalsPage/>
            <ChildrenPage/>
            <CommunicationPage/>
            <NumericInput/>
            <ShowPage/>
            <DerivedSignalPage/>
            <ControlFlow/>
            <Select/>
            <TextArea/>
            <ControlledForm/>
            <UncontrolledForm/>
            <ForExample/>
        }
    })
}
