use crate::state::{Entries, Entry};
use freya::prelude::*;

#[component]
pub fn Crud() -> Element {
    let items = Entries::default();
    rsx!(Cruds {  items })
}

#[component]
pub fn Cruds(items: Entries) -> Element {
    let items = use_signal(|| items);
    let mut filter = use_signal(|| String::new());
    let mut firstname = use_signal(|| String::new());
    let mut lastname = use_signal(|| String::new());
    let mut selected = use_signal::<Option<&Entry>>(|| None);
    // let mut selected = use_signal(|| items().list().first());

    rsx!(
        rect {
            direction: "vertical",
            width: "100%",
            height: "100%",
            padding: "24",
            spacing: "8",

            rect {
                direction: "horizontal",
                height: "auto",
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
                width: "100%",
                content: "flex",
                

                rect {
                    direction: "vertical",
                    // ScrollView {
                    rect {
                        direction: "vertical",

                        for item in items.read().list() {
                            rect {
                                width: "100%",
                                label { "{item}"}
                            }
                        }

                    }
                    rect {
                        direction: "horizontal",
                        label { "Name: "}
                        Input {
                            value: firstname,
                            onchange: move |t| {
                                firstname.set(t)
                            }
                        }
                    }
                    rect {
                        direction: "horizontal",
                        label { "Surname: "}
                        Input {
                            value: lastname,
                            onchange: move |t| {
                                lastname.set(t)
                            }
                        }
                    }
                }
            }
            rect {
                direction: "horizontal",

                Button {
                    onpress: move |_| {
                         items.read().clone().create(&firstname.read(), &lastname.read())
                    },
                    label {"Create"}
                }
                if selected.read().is_some() {
                    Button {
                        onpress: |_|  println!("clicked") ,
                        label {"Update"}
                    }
                    Button {
                        onpress: |_|  println!("clicked") ,
                        label {"Delete"}
                    }
                }
            }

        }
    )
}
