use yew_material_utils::prelude::*;

#[derive(Clone, Deserialize, Serialize)]
pub struct Extra {
    pub color: String,
}

pub fn initialize() {
    let mut ident = get_theme_ident(true);
    if ident == "" {
        ident = "dark".into();
    }
    init(
        ident,
        json!({
            "light": {
                "extra": {
                    "color": "#000"
                }
            },
            "dark": {
                "extra": {
                    "color": "#fff"
                }
            }
        }),
    );
}
