#[cfg(feature = "WiMoonset")]
use leptos::*;
#[cfg(feature = "WiMoonset")]
///This icon requires the feature `WiMoonset` to be enabled.
#[component]
pub fn Moonset(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M7.74,14.86c0-0.98,0.19-1.92,0.58-2.82c0.38-0.9,0.9-1.67,1.55-2.32c0.65-0.65,1.43-1.17,2.32-1.56&#xA;	c0.9-0.39,1.84-0.58,2.83-0.58h1.17c0.16,0.04,0.24,0.14,0.24,0.28l0.05,0.9c0.02,0.64,0.15,1.25,0.4,1.83s0.58,1.08,1,1.5&#xA;	c0.42,0.43,0.91,0.77,1.48,1.03c0.57,0.26,1.17,0.4,1.8,0.43l0.85,0.07c0.16,0,0.24,0.08,0.24,0.23v1.01&#xA;	c0.01,1.01-0.19,1.98-0.59,2.92h-2.05c0.51-0.74,0.83-1.59,0.97-2.53c-1.68-0.35-3.02-1.07-4.03-2.16s-1.59-2.37-1.75-3.83&#xA;	c-0.97,0.05-1.85,0.35-2.66,0.9c-0.8,0.55-1.42,1.24-1.87,2.08c-0.44,0.84-0.66,1.72-0.66,2.63c0,1.07,0.28,2.04,0.83,2.92H8.34&#xA;	C7.94,16.85,7.74,15.88,7.74,14.86z M7.99,20.89c0-0.26,0.1-0.49,0.3-0.69c0.18-0.18,0.41-0.27,0.68-0.27h3.22&#xA;	c0.11,0,0.2,0.02,0.28,0.08l2.35,2.22L17.21,20c0.07-0.05,0.17-0.08,0.29-0.08h3.3c0.27,0,0.5,0.09,0.69,0.28&#xA;	c0.19,0.19,0.29,0.42,0.29,0.68c0,0.27-0.1,0.5-0.29,0.69c-0.19,0.19-0.42,0.29-0.69,0.29h-2.68l-3.13,2.84&#xA;	c-0.12,0.09-0.24,0.09-0.34,0l-3.08-2.84h-2.6c-0.27,0-0.5-0.1-0.69-0.29C8.09,21.39,7.99,21.16,7.99,20.89z"
        /> < title > { title } < / title > < / svg >
    }
}
