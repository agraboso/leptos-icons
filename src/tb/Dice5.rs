#[cfg(feature = "TbDice5")]
use leptos::*;
#[cfg(feature = "TbDice5")]
///This icon requires the feature `TbDice5` to be enabled.
#[component]
pub fn Dice5(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-dice-5"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 3m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "8.5" cy = "8.5" r = ".5"
        fill = "currentColor" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "15.5"
        cy = "8.5" r = ".5" fill = "currentColor" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "15.5" cy = "15.5" r = ".5" fill =
        "currentColor" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "8.5" cy =
        "15.5" r = ".5" fill = "currentColor" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "12" cy = "12" r = ".5" fill = "currentColor"
        /> < title > { title } < / title > < / svg >
    }
}
