#[cfg(feature = "RiBusinessFillCreativeCommonsNc")]
use leptos::*;
#[cfg(feature = "RiBusinessFillCreativeCommonsNc")]
///This icon requires the feature `RiBusinessFillCreativeCommonsNc` to be enabled.
#[component]
pub fn CreativeCommonsNc(
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
        "M4.256 5.672l3.58 3.577a2.5 2.5 0 0 0 2 3.746L10 13h4l.09.008a.5.5 0 0 1 0 .984L14 14H8.5v2H11v2h2v-2h1c.121 0 .24-.009.357-.025l.173-.031 3.798 3.8A9.959 9.959 0 0 1 12 22C6.477 22 2 17.523 2 12c0-2.4.846-4.604 2.256-6.328zM12 2c5.523 0 10 4.477 10 10 0 2.4-.846 4.604-2.256 6.328l-3.579-3.577a2.5 2.5 0 0 0-2-3.745L14 11h-4l-.09-.008a.5.5 0 0 1 0-.984L10 10h5.5V8H13V6h-2v2h-1c-.121 0-.24.009-.356.025l-.173.031-3.799-3.8A9.959 9.959 0 0 1 12 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
