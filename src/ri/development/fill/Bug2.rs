#[cfg(feature = "RiDevelopmentFillBug2")]
use leptos::*;
#[cfg(feature = "RiDevelopmentFillBug2")]
///This icon requires the feature `RiDevelopmentFillBug2` to be enabled.
#[component]
pub fn Bug2(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M5.07 16A7.06 7.06 0 0 1 5 15v-1H3v-2h2v-1c0-.34.024-.673.07-1H3V8h2.674a7.03 7.03 0 0 1 2.84-3.072l-1.05-1.05L8.88 2.465l1.683 1.684a7.03 7.03 0 0 1 2.876 0l1.683-1.684 1.415 1.415-1.05 1.05A7.03 7.03 0 0 1 18.326 8H21v2h-2.07c.046.327.07.66.07 1v1h2v2h-2v1c0 .34-.024.673-.07 1H21v2h-2.674a7 7 0 0 1-12.652 0H3v-2h2.07zM9 10v2h6v-2H9zm0 4v2h6v-2H9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
