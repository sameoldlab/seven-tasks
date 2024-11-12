# [The 7 Tasks](https://eugenkiss.github.io/7guis/tasks/)

The tasks were selected by the following criteria. The task set should be as small as possible yet reflect as many typical (or fundamental or representative) challenges in GUI programming as possible. Each task should be as simple and self-contained as possible yet not too artificial. Preferably, a task should be based on existing examples as that gives the task more justification to be useful and there already will be at least one reference implementation.

Below, a description of each task highlighted with the challenges it reflects and a screenshot of the resulting GUI application in Java/Swing is given.

For a live version of the tasks see my React/MobX implementation.
Counter

Challenge: Understanding the basic ideas of a language/toolkit.

The task is to build a frame containing a label or read-only textfield T and a button B. Initially, the value in T is “0” and each click of B increases the value in T by one.

Counter serves as a gentle introduction to the basics of the language, paradigm and toolkit for one of the simplest GUI applications imaginable. Thus, Counter reveals the required scaffolding and how the very basic features work together to build a GUI application. A good solution will have almost no scaffolding.
Temperature Converter

Challenges: bidirectional data flow, user-provided text input.

The task is to build a frame containing two textfields TC and TF representing the temperature in Celsius and Fahrenheit, respectively. Initially, both TC and TF are empty. When the user enters a numerical value into TC the corresponding value in TF is automatically updated and vice versa. When the user enters a non-numerical string into TC the value in TF is not updated and vice versa. The formula for converting a temperature C in Celsius into a temperature F in Fahrenheit is C = (F - 32) * (5/9) and the dual direction is F = C * (9/5) + 32.

Temperature Converter increases the complexity of Counter by having bidirectional data flow between the Celsius and Fahrenheit inputs and the need to check the user input for validity. A good solution will make the bidirectional dependency very clear with minimal boilerplate code.

Temperature Converter is inspired by the Celsius/Fahrenheit converter from the book Programming in Scala. It is such a widespread example—sometimes also in the form of a currency converter—that one could give a thousand references. The same is true for the Counter task.
Flight Booker

Challenge: Constraints.

The task is to build a frame containing a combobox C with the two options “one-way flight” and “return flight”, two textfields T1 and T2 representing the start and return date, respectively, and a button B for submitting the selected flight. T2 is enabled iff C’s value is “return flight”. When C has the value “return flight” and T2’s date is strictly before T1’s then B is disabled. When a non-disabled textfield T has an ill-formatted date then T is colored red and B is disabled. When clicking B a message is displayed informing the user of his selection (e.g. “You have booked a one-way flight on 04.04.2014.”). Initially, C has the value “one-way flight” and T1 as well as T2 have the same (arbitrary) date (it is implied that T2 is disabled).

The focus of Flight Booker lies on modelling constraints between widgets on the one hand and modelling constraints within a widget on the other hand. Such constraints are very common in everyday interactions with GUI applications. A good solution for Flight Booker will make the constraints clear, succinct and explicit in the source code and not hidden behind a lot of scaffolding.

Flight Booker is directly inspired by the Flight Booking Java example in Sodium with the simplification of using textfields for date input instead of specialized date picking widgets as the focus of Flight Booker is not on specialized/custom widgets.
Timer

Challenges: concurrency, competing user/signal interactions, responsiveness.

The task is to build a frame containing a gauge G for the elapsed time e, a label which shows the elapsed time as a numerical value, a slider S by which the duration d of the timer can be adjusted while the timer is running and a reset button R. Adjusting S must immediately reflect on d and not only when S is released. It follows that while moving S the filled amount of G will (usually) change immediately. When e ≥ d is true then the timer stops (and G will be full). If, thereafter, d is increased such that d > e will be true then the timer restarts to tick until e ≥ d is true again. Clicking R will reset e to zero.

Timer deals with concurrency in the sense that a timer process that updates the elapsed time runs concurrently to the user’s interactions with the GUI application. This also means that the solution to competing user and signal interactions is tested. The fact that slider adjustments must be reflected immediately moreover tests the responsiveness of the solution. A good solution will make it clear that the signal is a timer tick and, as always, has not much scaffolding.

Timer is directly inspired by the timer example in the paper Crossing State Lines: Adapting Object-Oriented Frameworks to Functional Reactive Languages.
CRUD

Challenges: separating the domain and presentation logic, managing mutation, building a non-trivial layout.

The task is to build a frame containing the following elements: a textfield Tprefix, a pair of textfields Tname and Tsurname, a listbox L, buttons BC, BU and BD and the three labels as seen in the screenshot. L presents a view of the data in the database that consists of a list of names. At most one entry can be selected in L at a time. By entering a string into Tprefix the user can filter the names whose surname start with the entered prefix—this should happen immediately without having to submit the prefix with enter. Clicking BC will append the resulting name from concatenating the strings in Tname and Tsurname to L. BU and BD are enabled iff an entry in L is selected. In contrast to BC, BU will not append the resulting name but instead replace the selected entry with the new name. BD will remove the selected entry. The layout is to be done like suggested in the screenshot. In particular, L must occupy all the remaining space.

CRUD (Create, Read, Update and Delete) represents a typical graphical business application. The primary challenge is the separation of domain and presentation logic in the source code that is more or less forced on the implementer due to the ability to filter the view by a prefix. Traditionally, some form of MVC pattern is used to achieve the separation of domain and presentation logic. Also, the approach to managing the mutation of the list of names is tested. A good solution will have a good separation between the domain and presentation logic without much overhead (e.g. in the form of toolkit specific concepts or language/paradigm concepts), a mutation management that is fast but not error-prone and a natural representation of the layout (layout builders are allowed, of course, but would increase the overhead).

CRUD is directly inspired by the crud example in the blog post FRP - Three principles for GUI elements with bidirectional data flow.
Circle Drawer

Challenges: undo/redo, custom drawing, dialog control*.

The task is to build a frame containing an undo and redo button as well as a canvas area underneath. Left-clicking inside an empty area inside the canvas will create an unfilled circle with a fixed diameter whose center is the left-clicked point. The circle nearest to the mouse pointer such that the distance from its center to the pointer is less than its radius, if it exists, is filled with the color gray. The gray circle is the selected circle C. Right-clicking C will make a popup menu appear with one entry “Adjust diameter..”. Clicking on this entry will open another frame with a slider inside that adjusts the diameter of C. Changes are applied immediately. Closing this frame will mark the last diameter as significant for the undo/redo history. Clicking undo will undo the last significant change (i.e. circle creation or diameter adjustment). Clicking redo will reapply the last undoed change unless new changes were made by the user in the meantime.

Circle Drawer’s goal is, among other things, to test how good the common challenge of implementing an undo/redo functionality for a GUI application can be solved. In an ideal solution the undo/redo functionality comes for free resp. just comes out as a natural consequence of the language / toolkit / paradigm. Moreover, Circle Drawer tests how dialog control*, i.e. keeping the relevant context between several successive GUI interaction steps, is achieved in the source code. Last but not least, the ease of custom drawing is tested.

* Dialog control is explained in more detail in the paper Developing GUI Applications: Architectural Patterns Revisited starting on page seven. The term describes the challenge of retaining context between successive GUI operations.
Cells

Challenges: change propagation, widget customization, implementing a more authentic/involved GUI application.

The task is to create a simple but usable spreadsheet application. The spreadsheet should be scrollable. The rows should be numbered from 0 to 99 and the columns from A to Z. Double-clicking a cell C lets the user change C’s formula. After having finished editing the formula is parsed and evaluated and its updated value is shown in C. In addition, all cells which depend on C must be reevaluated. This process repeats until there are no more changes in the values of any cell (change propagation). Note that one should not just recompute the value of every cell but only of those cells that depend on another cell’s changed value. If there is an already provided spreadsheet widget it should not be used. Instead, another similar widget (like JTable in Swing) should be customized to become a reusable spreadsheet widget.

Cells is a more authentic and involved task that tests if a particular approach also scales to a somewhat bigger application. The two primary GUI-related challenges are intelligent propagation of changes and widget customization. Admittedly, there is a substantial part that is not necessarily very GUI-related but that is just the nature of a more authentic challenge. A good solution’s change propagation will not involve much effort and the customization of a widget should not prove too difficult. The domain-specific code is clearly separated from the GUI-specific code. The resulting spreadsheet widget is reusable.

Cells is directly inspired by the SCells spreadsheet example from the book Programming in Scala. Please refer to the book (or the implementations in this repository) for more details especially with respect to the not directly GUI-related concerns like parsing and evaluating formulas and the precise syntax and semantics of the spreadsheet language.
