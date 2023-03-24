#[cfg(feature = "SiTrove")]
use leptos::*;
#[cfg(feature = "SiTrove")]
///This icon requires the feature `SiTrove` to be enabled.
#[component]
pub fn Trove(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M11.922 8.43c-1.393.035-2.506 1.213-2.486 2.606a2.534 2.534 0 0 0 1.763 2.392l.25-1.123a1.424 1.424 0 0 1-.875-1.307c0-.883.816-1.594 1.72-1.396.56.122.994.58 1.095 1.143a1.419 1.419 0 0 1-.854 1.558l.25 1.123a2.537 2.537 0 0 0 1.764-2.428 2.557 2.557 0 0 0-2.627-2.568zM.189 8.5a.138.138 0 0 0-.136.137v.776c0 .07.058.134.136.134h1.016v3.797a.14.14 0 0 0 .135.135h.853a.141.141 0 0 0 .137-.135V9.547h1.016a.136.136 0 0 0 .136-.134v-.776a.138.138 0 0 0-.136-.137zm4.504 0a.136.136 0 0 0-.134.137v4.707c0 .071.056.137.134.137h.846a.143.143 0 0 0 .137-.137v-1.812h.49l.945 1.884a.13.13 0 0 0 .114.065h.947c.12 0 .17-.115.121-.207l-.975-1.807c.62-.249 1.053-.775 1.053-1.43 0-.846-.697-1.537-1.55-1.537zm10.356 0c-.107 0-.164.094-.121.186l2.218 4.787a.136.136 0 0 0 .122.078h.072c.057 0 .1-.035.12-.078l2.233-4.787c.043-.092-.014-.186-.12-.186h-.946a.14.14 0 0 0-.121.079l-1.174 2.597h-.043L16.115 8.58a.14.14 0 0 0-.12-.079zm5.805 0a.138.138 0 0 0-.137.137v4.707c0 .071.058.137.137.137h2.943a.136.136 0 0 0 .135-.137v-.775a.134.134 0 0 0-.135-.135h-1.97v-.96h1.622a.134.134 0 0 0 .135-.136v-.775a.14.14 0 0 0-.135-.135h-1.623v-.88h1.97a.134.134 0 0 0 .136-.135v-.776a.136.136 0 0 0-.135-.137zM5.676 9.518h1.053c.291 0 .539.242.539.541 0 .299-.248.568-.54.568H5.677zM.135 14.526A.134.134 0 0 0 0 14.66v.775c0 .071.056.135.135.135h10.588l.232-1.045zm12.896 0 .233 1.045h10.601a.134.134 0 0 0 .135-.135v-.775a.134.134 0 0 0-.135-.135z"
        /> < title > { title } < / title > < / svg >
    }
}
