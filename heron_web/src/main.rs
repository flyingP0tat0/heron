#![recursion_limit="512"]

use heron::heron;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

fn main() {
    yew::start_app::<Model>();
}

pub struct Model {
    square: String,
    precision: String,
    root: f32
}

pub enum Message {
    Calculcate,
    Input(Input, String)
}

pub enum Input {
    Square,
    Precision
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            square: String::new(),
            precision: String::new(),
            root: 0.0
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::Calculcate => {
                let square = self.square.parse::<f32>().expect("Error parsing input!");
                let precision = self.precision.parse::<f32>().expect("Error parsing input!");
                self.root = heron(square, precision);
            }
            Message::Input(input_type, input) => match input_type {
                Input::Square => self.square = input,
                Input::Precision => self.precision = input
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input value=&self.square oninput=|event| Message::Input(Input::Square, event.value) placeholder="square"></input>
                <br></br>
                <input value=&self.precision oninput=|event| Message::Input(Input::Precision, event.value) placeholder="precision"></input>
                <br></br>
                <button onclick=|_| Message::Calculcate>{ "calculcate" }</button>
                <br></br>
                <p>{ "square root: " }<b>{ &self.root }</b></p>
            </div>
        }
    }
}
