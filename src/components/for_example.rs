use leptos::*;

#[component]
pub fn ForExample() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "0".into(),
            value: create_rw_signal(0),
        },
        DatabaseEntry {
            key: "1".into(),
            value: create_rw_signal(1),
        },
        DatabaseEntry {
            key: "2".into(),
            value: create_rw_signal(2),
        },
    ]);
    view! {
        <details>
            <summary>For and Collect View</summary>
            <div>
                <button on:click=move |_| {
                    set_data
                        .update(|data| {
                            let len = data.len();
                            data.push(DatabaseEntry {
                                key: len.to_string(),
                                value: create_rw_signal(len as i32),
                            })
                        })
                }>Add</button>
                <For
                    each=move || data.get().into_iter().enumerate()
                    key=|(_, entry)| entry.key.clone()
                    children=move |(index, child)| {
                        view! {
                            <p>
                                <button on:click=move |_| {
                                    child.value.update(|val| *val += 1)
                                }>"+"</button>
                                " "
                                {child.value}
                                " "
                                <button on:click=move |_| {
                                    set_data
                                        .update(|data| {
                                            data.retain(|el| { el.key != index.to_string() })
                                        })
                                }>"🗑"</button>
                            </p>
                            " "
                        }
                    }
                />

                <div>
                    {data
                        .get()
                        .clone()
                        .into_iter()
                        .map(|el| el.value)
                        .collect_view()}
                </div>
            </div>
        </details>
    }
}

#[derive(Clone, Debug)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}
