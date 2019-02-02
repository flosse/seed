#[macro_use]
extern crate seed;
use seed::prelude::*;

#[derive(Clone)]
struct Model {
    page: Page,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Foo,
    About,
}

impl Page {
    fn name(&self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Foo => "Foo",
            Page::About => "About",
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self { page: Page::Home }
    }
}

#[derive(Clone)]
enum Msg {
    SetPage(Page),
}

fn update(msg: Msg, model: Model) -> Update<Model> {
    match msg {
        Msg::SetPage(page) => Render(Model { page, ..model }),
    }
}

fn menu(model: &Model) -> El<Msg> {
    aside![
        attrs! {"class"=> "menu";},
        nav![
            menu_entry(model, Page::Home),
            menu_entry(model, Page::Foo),
            menu_entry(model, Page::About),
        ]
    ]
}

fn menu_entry(model: &Model, page: Page) -> El<Msg> {
    let active = model.page == page;

    a![
        attrs! {
            "href"=>"#";
            "class" => if active { "active" } else { "" };
        },
        span![page.name()],
        simple_ev("click", Msg::SetPage(page)),
    ]
}
fn view(_: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    div![
        menu(model),
        main![
            match model.page {
                Page::Home => div![
                    div!["home content 0"],
                    div!["home content 1"],
                    div!["home content 2"],
                ],
                Page::Foo => div!
                    [h2!["Foo"],
                    div!["foo content 0"],
                    div!["foo content 1"],
                ],
                Page::About => div![
                    p!["About"]
                ],
            }
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
