use leptos::*;

#[component]
pub fn WorkingWithSignalsPage() -> impl IntoView {
    let (first, _) = create_signal("Haha");
    let (second, _) = create_signal("Hehe");
    let (third, _) = create_signal("Hoho");
    // the with! macro is handy for creating a closure where many signals are used
    let sum = move || with!(|first, second, third| format!("{first} {second} {third}"));
    view! {
        <details>
            <summary>"Working With Signals"</summary>
            {sum}
        </details>
    }
}
