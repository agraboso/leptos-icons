#[cfg(feature = "SiAmazonapigateway")]
use leptos::*;
#[cfg(feature = "SiAmazonapigateway")]
///This icon requires the feature `SiAmazonapigateway` to be enabled.
#[component]
pub fn Amazonapigateway(
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
        "M9.456 18.7h1.258v-.865H9.456Zm2.115 0h1.286v-.865h-1.286ZM6.43 1.127.857 3.892v17.19l5.572 1.885Zm.857 5.47v11.238H8.57v.865H7.286v4.868a.434.434 0 0 1-.429.432.425.425 0 0 1-.136-.022L.292 21.804a.432.432 0 0 1-.292-.41V3.623c0-.164.093-.315.24-.388L6.668.045a.424.424 0 0 1 .415.02.433.433 0 0 1 .203.367v5.3H8.57v.865ZM13.714 18.7H15v-.865h-1.286Zm.028-12.103H15v-.864h-1.258Zm-2.143 0h1.258v-.864H11.6Zm-2.143 0h1.258v-.864H9.456Zm13.687-2.705L17.57 1.127v21.84l5.572-1.884ZM24 21.394c0 .186-.117.35-.292.41l-6.429 2.174a.425.425 0 0 1-.386-.06.434.434 0 0 1-.179-.35V18.7h-.829v-.865h.83V6.597h-.83v-.864h.83v-5.3c0-.15.076-.289.202-.368a.424.424 0 0 1 .415-.02l6.428 3.19c.147.073.24.224.24.388ZM13.257 9.346l-.8-.31-2.143 5.618.8.31Zm2.903 2.744a.434.434 0 0 0 0-.612L14.446 9.75l-.606.612 1.411 1.423-1.411 1.423.606.611Zm-6.606 1.728L7.84 12.09a.434.434 0 0 1 0-.612L9.554 9.75l.606.612-1.411 1.423 1.411 1.423Z"
        /> < title > { title } < / title > < / svg >
    }
}
