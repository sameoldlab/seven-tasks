use state::crud::Entry;
use freya::prelude::*;

#[component]
pub fn Crud() -> Element {
    let mut items = use_signal(|| {
        vec![
            Entry::new("Hans".to_string(), "Emil".to_string()),
            Entry::new("Max".to_string(), "Musterman".to_string()),
            Entry::new("Roman".to_string(), "Tisch".to_string()),
        ]
    });

    let mut filter =    use_signal(|| String::new());
    let mut firstname = use_signal(|| String::new());
    let mut lastname =  use_signal(|| String::new());
    // let selected =  use_signal(|| (*items.read().clone()).first());
    // let selected =  use_signal(|| items.read().clone().first().cloned());
    let mut selected_idx = use_signal::<Option<usize>>(|| {
        if items.read().clone().len() > 0 {
            Some(0)
        } else { None }
    });

    rsx!(
        rect {
            direction: "vertical",
            width: "100%",
            height: "100%",
            content: "flex",
            padding: "24",
            spacing: "8",

            rect {
                direction: "horizontal",
                spacing: "8",
                cross_align: "center",
                label { "Filter prefix: "}
                Input {
                    value: filter,
                    onchange: move |t| {
                        filter.set(t)
                    }
                }
            }
            rect {
                direction: "horizontal",
                spacing: "12",
                height: "flex(1)",
                width: "100%",
                content: "flex",
                
                rect {
                    background: "#222222",
                    height: "100%",
                    width: "flex(1)",

                    ScrollView {
                        direction: "vertical",

                        for (i, item) in items.read().iter().enumerate().filter(|(_, item)| {
                            item.firstname.to_lowercase().contains(&filter().trim().to_lowercase())
                            || item.lastname.to_lowercase().contains(&filter().trim().to_lowercase())
                        }) {
                            Button {
                                theme: theme_with!(ButtonTheme {
                                    background: (if selected_idx().or(Some(0)) == Some(i) { "#454173"} else {"transparent"}).into(),
                                    hover_background: "#555555".into(),
                                    border_fill: "transparent".into(),
                                    margin: "0".into(),
                                    corner_radius: "0".into(),
                                    width: "100%".into(),
                                    padding: "4".into(),
                                }),
                                onpress:  move |_| {
                                        selected_idx.set(Some(i));
                                        if firstname().is_empty() { firstname.set(items()[i].firstname.clone()); }
                                        if lastname().is_empty() { lastname.set(items()[i].lastname.clone()); }
                                },
                                label { "{item}"}
                            }
                        }

                    }
                }

                rect {
                    direction: "vertical",
                    width: "flex(1)",
                    spacing: "8",

                    rect {
                        direction: "horizontal",
                        spacing: "8",
                        cross_align: "center",
                        
                        label {
                              width: "90", 
                             "Name: ",
                        }
                        Input {
                            value: firstname,
                            onchange: move |t| firstname.set(t), 
                        }
                    }
                    rect {
                        direction: "horizontal",
                        spacing: "8",
                        cross_align: "center",
                        label {
                            width: "90", 
                            "Surname: ",
                        }
                        Input {
                            value: lastname,
                            onchange: move |t| lastname.set(t),
                        }
                    }
                }
            }
            rect {
                direction: "horizontal",
                spacing: "4",

                Button {
                    onpress: move |_|  {
                        let first = firstname().trim().to_string();
                        let last = lastname().trim().to_string();
                        if first.is_empty() || last.is_empty() {return}
                        items.write().push(Entry::new(first, last));
                    } ,
                    label {"Create"}
                }
                if selected_idx.read().is_some() {
                    Button {
                        onpress: move |_|  {
                            let pos = selected_idx().unwrap();
                            items.write()[pos].firstname = firstname.to_string();
                            items.write()[pos].lastname = lastname.to_string();
                        },
                        label {"Update"}
                    }
                    Button {
                        onpress: move |_|  {
                            let pos = selected_idx().unwrap();
                            items.write().swap_remove(pos);
                        },
                        label {"Delete"}
                    }
                }
            }
        }
    )
}
