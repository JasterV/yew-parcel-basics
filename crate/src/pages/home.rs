use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Home {
    link: ComponentLink<Self>,
    state: i32,
}

pub enum Msg {
    AddOne,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home { state: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.state += 1;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::AddOne);
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Times clicked: "}{self.state}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <button onclick=onclick>{"Add One!"}</button>
                </Item>
            </Container>
        }
    }
}
