use crate::theme::*;

use yew::prelude::*;
use yew_material::prelude::*;
use yew_material_utils::prelude::*;
use yew_material_utils::theme::change_theme;

pub enum Msg {
    ChangeTheme,
}

pub struct Index {
    link: ComponentLink<Self>,
    theme: String,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            theme: get_theme_ident(false),
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::ChangeTheme => {
                self.theme = match self.theme.as_str() {
                    "light" => "dark".into(),
                    _ => "light".into(),
                };
                change_theme(self.theme.to_string());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let theme = to_theme::<Extra>();
        let style = Style::easy(json!({
            "content": {
                "color": "#9d58ff",
                "& h1": {
                    "color": theme.extra.color,
                },
                "& a": {
                    "margin": "0 10px",
                },
                "& .describe": {
                    "font-size": "18px",
                },
            }
        }));
        html! {
            <Appbar>
                <span slot="title">{"Yew Material-UI"}</span>
                <IconButton
                    slot="actionItems"
                    color="#fff"
                    on=if self.theme == "light" {true} else {false}
                    on_icon="brightness_7"
                    off_icon="brightness_4"
                    onclick=self.link.callback(|_| Msg::ChangeTheme)
                />
                <Flex class=style.item("content") direction="column" justify="center" align="center" padding="80px">
                    <h1>{"Yew Material App"}</h1>
                    <p>
                        <a target="_blank" href="https://www.yew-material.cn">{"website"}</a>
                        <a target="_blank" href="https://github.com/alibaba-inc/yew-material">{"github"}</a>
                    </p>
                    <div class="describe">{"yew-material is a material-ui framework for yew"}</div>
                </Flex>
            </Appbar>
        }
    }
}
