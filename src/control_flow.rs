use leptos::*;

#[component]
pub fn ControlFlow() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;
    let increment = move |_| set_value.update(|v| *v += 1);

    view! {
        <details>
            <summary>Control Flow</summary>
            <p>
                <button on:click=increment>"+"</button>
                " "
                {value}
                {move || if is_odd() { " Odd" } else { " Even" }}
            </p>
        </details>
    }
}
