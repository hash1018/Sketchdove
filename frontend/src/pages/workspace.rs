use std::{cell::RefCell, rc::Rc};

use lib::{figure::Figure, message::ServerMessage};
use yew::{html, Component, Context, Properties};
use yew_agent::{Bridge, Bridged};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    base::DrawModeType,
    client::{event_bus::EventBus, websocket_service::WebsocketService},
    components::{
        chat::Chat,
        draw_area::DrawArea,
        login::{Login, LoginNotifyMessage},
        title_bar::TitleBar,
        tool_box::ToolBox,
    },
    pages::app::user_name,
};

use super::app::{set_user_name, Route};

pub enum WorkSpaceMessage {
    HandleServerMessage(ServerMessage),
    HandleChildRequest(ChildRequestType),
    RequestInit,
    HandleLoginNotifyMessage(LoginNotifyMessage),
}

pub enum ChildRequestType {
    Leave,
    ShowChat(bool),
    ChangeMode(DrawModeType),
    AddFigure(Box<dyn Figure>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct WorkspaceProps {
    pub id: String,
}

pub struct Workspace {
    wss: Option<WebsocketService>,
    _event_bus: Option<Box<dyn Bridge<EventBus>>>,
    show_chat: bool,
    current_mode: DrawModeType,
    figures: Rc<FigureList>,
    logined: bool,
}

impl Component for Workspace {
    type Message = WorkSpaceMessage;
    type Properties = WorkspaceProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let user_name = user_name();
        if user_name.is_some() {
            let link = ctx.link();
            link.send_message(WorkSpaceMessage::RequestInit);
        }

        Self {
            wss: None,
            _event_bus: None,
            show_chat: false,
            current_mode: DrawModeType::SelectMode,
            figures: Rc::new(FigureList::new()),
            logined: false,
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(wss) = self.wss.as_ref() {
            wss.disconnect();
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WorkSpaceMessage::RequestInit => {
                let user_name = user_name().unwrap();
                let room_id = ctx.props().id.clone();
                log::info!("connect websocket user_name {user_name}, room_id {room_id}");

                (self.wss, self._event_bus) = init(ctx);
                self.logined = true;

                if let Some(wss) = self.wss.as_ref() {
                    wss.send(lib::message::ClientMessage::Join(room_id, user_name));
                }

                return true;
            }
            WorkSpaceMessage::HandleServerMessage(server_message) => {
                log::debug!("received message from event_bus {server_message:?}");
            }
            WorkSpaceMessage::HandleChildRequest(request) => match request {
                ChildRequestType::Leave => {
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.push(&Route::Main);
                }
                ChildRequestType::ShowChat(show) => {
                    self.show_chat = show;
                    return true;
                }
                ChildRequestType::ChangeMode(mode) => {
                    if mode != self.current_mode {
                        self.current_mode = mode;
                        return true;
                    }
                }
                ChildRequestType::AddFigure(figure) => {
                    self.figures.push(figure);
                    return true;
                }
            },
            WorkSpaceMessage::HandleLoginNotifyMessage(msg) => match msg {
                LoginNotifyMessage::EnterRoom(name, _room_id) => {
                    set_user_name(Some(name));
                    let link = ctx.link();
                    link.send_message(WorkSpaceMessage::RequestInit);
                }
            },
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        if self.logined {
            self.show_draw_area(ctx)
        } else {
            let handler = ctx
                .link()
                .callback(WorkSpaceMessage::HandleLoginNotifyMessage);
            html! {
               <div>
                   <Login {handler} hide_create_button = {true} />
               </div>
            }
        }
    }
}

impl Workspace {
    fn show_draw_area(&self, ctx: &yew::Context<Workspace>) -> yew::Html {
        let handler = ctx.link().callback(WorkSpaceMessage::HandleChildRequest);
        let show_chat = self.show_chat;
        let current_mode = self.current_mode;
        let handler_clone = handler.clone();
        let handler_clone2 = handler.clone();
        let figures = self.figures.clone();

        html! {
            <body>
                <div class="top"> <TitleBar {handler} {show_chat} /> </div>
                <div class="content">
                    <DrawArea handler = {handler_clone} {current_mode} {figures} />
                    <div class="left"> <ToolBox handler = {handler_clone2} {current_mode} /> </div>
                    if show_chat {
                        <div class="chat_position"> <Chat /> </div>
                    }
                </div>
            </body>
        }
    }
}

fn init(ctx: &Context<Workspace>) -> (Option<WebsocketService>, Option<Box<dyn Bridge<EventBus>>>) {
    let wss = WebsocketService::new();
    wss.connect().unwrap();
    let callback = {
        let link = ctx.link().clone();
        move |e| link.send_message(WorkSpaceMessage::HandleServerMessage(e))
    };

    (Some(wss), Some(EventBus::bridge(Rc::new(callback))))
}

#[derive(Default)]
pub struct FigureList {
    list: Rc<RefCell<Vec<Box<dyn Figure>>>>,
}

impl PartialEq for FigureList {
    fn eq(&self, other: &Self) -> bool {
        self.list.borrow().len() == other.list.borrow().len()
    }
}

impl FigureList {
    fn new() -> FigureList {
        FigureList {
            list: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn push(&self, figure: Box<dyn Figure>) {
        self.list.borrow_mut().push(figure);
    }

    pub fn list(&self) -> Rc<RefCell<Vec<Box<dyn Figure>>>> {
        self.list.clone()
    }
}
