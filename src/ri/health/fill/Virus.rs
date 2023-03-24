#[cfg(feature = "RiHealthFillVirus")]
use leptos::*;
#[cfg(feature = "RiHealthFillVirus")]
///This icon requires the feature `RiHealthFillVirus` to be enabled.
#[component]
pub fn Virus(
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
        "none" d = "M0 0H24V24H0z" />< path d =
        "M13.717 1.947l3.734 1.434-.717 1.867-.934-.359-.746 1.945c.779.462 1.444 1.094 1.945 1.846l1.903-.847-.407-.914 1.827-.813 1.627 3.654-1.827.813-.407-.913-1.902.847c.122.477.187.978.187 1.493 0 .406-.04.803-.117 1.187l1.944.746.358-.933 1.868.717-1.434 3.734-1.867-.717.358-.933-1.944-.747c-.462.779-1.094 1.444-1.846 1.945l.847 1.903.914-.407.813 1.827-3.654 1.627-.813-1.827.913-.407-.847-1.902c-.477.122-.978.187-1.493.187-.407 0-.804-.04-1.188-.118l-.746 1.945.934.358-.717 1.868-3.734-1.434.717-1.867.932.358.748-1.944C8.167 16.704 7.502 16.072 7 15.32l-1.903.847.407.914-1.827.813-1.627-3.654 1.827-.813.406.914 1.903-.848C6.065 13.016 6 12.515 6 12c0-.406.04-.803.117-1.187l-1.945-.746-.357.933-1.868-.717L3.381 6.55l1.867.717-.359.933 1.945.747C7.296 8.167 7.928 7.502 8.68 7l-.847-1.903-.914.407-.813-1.827L9.76 2.051l.813 1.827-.913.407.847 1.902C10.984 6.065 11.485 6 12 6c.406 0 .803.04 1.187.117l.745-1.945L13 3.815l.717-1.868zm-3.583 11.285c-.276.478-.112 1.09.366 1.366s1.09.112 1.366-.366.112-1.09-.366-1.366-1.09-.112-1.366.366zM14 11c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1zm-3.5-1.598c-.478.276-.642.888-.366 1.366.276.478.888.642 1.366.366.478-.276.642-.888.366-1.366-.276-.478-.888-.642-1.366-.366z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
