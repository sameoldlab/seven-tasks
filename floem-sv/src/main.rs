use floem::prelude::*;

fn main() {
    floem::launch(view);
}

fn view() -> impl IntoView {
    let mut count = RwSignal::new(0);

    h_stack((
        button("-").action(move || count -=1),
        label(move || format!("{count}")),
        button("+").action(move || count +=1),
    ))
}
