use crate::state::Entry;
use freya::prelude::*;

#[component]
pub fn Crud() -> Element {
    let mut items = use_signal(|| {
        vec![
            Entry::new("Hans", "Emil"),
            Entry::new("Max", "Musterman"),
            Entry::new("Roman", "Tisch"),
        ]
    });

    let mut filter =    use_signal(|| String::new());
    let mut firstname = use_signal(|| String::new());
    let mut lastname =  use_signal(|| String::new());
    // let selected =  use_signal(|| (*items.read().clone()).first());
    // let selected =  use_signal(|| items.read().clone().first().cloned());
    let selected_idx = use_signal::<Option<usize>>(|| {
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
                    padding: "8",

                    ScrollView {
                        direction: "vertical",

                        for item in items.read().iter() {
                            rect {
                                width: "100%",
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
                        items.write().push(Entry::new(firstname().as_str(), lastname().as_str()));
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
