## Initial Install
~410 dependencies. First build took 1m42sec for me. Comes with a couple features (fira-code, wgpu, tiny-skia) enabled.

## Learning Iced
On November 12th, 2024 the iced book is still empty. There is a YouTube video by hector using an older version of iced. but in the 0.13.1 Sandbox has been replaced with Application. I don't know what else has changed. 
This seems like a more useful resource than the official book. https://jl710.github.io/iced-guide/

## Temp Converter
This is my first time using an Elm-architecture framework and I'm not sure what to think yet. The temp converter is still very simple but I can already see why some people might be turned off by the boilderplate. Typing into an input and having it's value change involves 6 references to celsius.

```rust
#[derive(Default)]
pub struct Converter {
    celsius: String, 
}

#[derive(Debug, Clone)]
enum Message {
    CelsiusChanged(String),
}
fn update(state: &mut Converter, message: Message) {
    match message {
        Message::CelsiusChanged(string) => {
            state.celsius = string;
        }
    }
}
fn view(state: &Converter) -> Element<Message> {
    row![
      text_input("0", &state.celsius).on_input(Message::CelsiusChanged),
    ].into()
}
```
whereas in svelte this would be:
```svelte
<script>
  let celsius = $state(0)
</script>

<input bind:value={celsius}/>
```
Is this an unfair comparison because svelte is around 7 years old at this point and has gone through several version while iced is still exprimental? Maybe.
Is this an unfair comparison because I'm comparing elm in rust to a declarative UI DSL that compiles to javascript? No. This is part of the choice people regularly make when they consider Electron vs more "native" toolkits. I'll need to try more rust and more Elm before making any grand statements, but expecting the same level of productivity available in recent signal-based, hot reloading JS frameworks while writing pure rust seems... unlikely.    

A more complete example from Temp Converter shows the relationsip between the two values:
```rust
#[derive(Default)]
pub struct Fb {
  celsius: String,
  fahrenheit: String,
}

#[derive(Debug, Clone)]
enum Message {
    CelsiusChanged(String),
    FahrenheitChanged(String),
}
match message {
        Message::CelsiusChanged(string) => {
            state.celsius = string;
            match state.celsius.parse::<f64>() {
                Ok(celsius) => {
                    state.fahrenheit = (celsius * (9.0 / 5.0) + 32.0).round().to_string()
                }

                Err(err) => println!("{:#?}", err),
            }
        }
        Message::FahrenheitChanged(string) => {
            state.fahrenheit = string;
            match state.fahrenheit.parse::<f64>() {
                Ok(fahrenheit) => {
                    state.celsius = ((fahrenheit - 32.0) * (5.0 / 9.0)).round().to_string()
                }

                Err(err) => println!("{:#?}", err),
            }
        }
    }
```
> Temperature Converter increases the complexity of Counter by having bidirectional data flow between the Celsius and Fahrenheit inputs and the need to check the user input for validity. A good solution will make the bidirectional dependency very clear with minimal boilerplate code.

...

Maybe my rust is bad, or elm shines in larger scale applications, or iced can render a million items at 60fps, idk. Right now it looks like a lot of boilerplate with hidden dependencies. 

## Flight
iced does not have dropdowns. yes, there are no dropdowns. you're confused aren't you? how could a default component set not include dropdowns? ok, fine. they're called 'picklists'. yeah, picklists. a library with almost no docs, reduced their ability for the words to document themselves. afte(r a minute of googling, it appears salesforce also employs this term).

[Move to general notes] 
Rust has a tiny standard library in comparison to all the languages I'm familiar with. Maybe this is a normal for the C/C++-like low-level languages? Serde, reqwest, and tokio are too complex to casually roll-your-own, but have become the defaults while staying separate from stdlib (and for tokio having an ecosystem around its sepcific runtime). With dates, I've come across time, chrono, and icu so far (blessed.rs recommends the first two, but there's a hundred more on lib.rs).

I picked time because the package was about 100kb smaller than chrono. This [discussion post](https://users.rust-lang.org/t/the-state-of-time-in-rust-leaps-and-bounds/107620/12) has more details on time libraries in rust. They get in the weeds about each package's supprot for leap seconds, something I'd never thought about before now. By the time (no pun intended) I got around to reading it I'd almost finished flight booker. A common phenomenon where coin flips can be twice as productive as bike shedding package options.

Beyond picking a library for my project I also wonder how it affects any UI library trying to make a good date picker widget.
