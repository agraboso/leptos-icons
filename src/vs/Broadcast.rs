#[cfg(feature = "VsBroadcast")]
use leptos::*;
#[cfg(feature = "VsBroadcast")]
///This icon requires the feature `VsBroadcast` to be enabled.
#[component]
pub fn Broadcast(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.667 2.011A6 6 0 0 1 8 1a6.007 6.007 0 0 1 6 6 6 6 0 0 1-3.996 5.655v-.044c.016-.014.031-.03.046-.045a1.48 1.48 0 0 0 .434-1.046v-.137A5.042 5.042 0 0 0 12.19 4.2a5.04 5.04 0 1 0-6.69 7.176v.144a1.48 1.48 0 0 0 .48 1.09v.04A5.999 5.999 0 0 1 4.667 2.01z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.343 11.86a.48.48 0 0 1-.34.14v2.52a.48.48 0 0 1-.48.48H7.46c.011 0-.004-.004-.034-.012-.075-.02-.241-.064-.305-.129a.48.48 0 0 1-.141-.34V12a.48.48 0 0 1-.48-.48V9.5a1 1 0 0 1 1-1h.984a1 1 0 0 1 1 1v2.02a.48.48 0 0 1-.137.335l-.004.004z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.64 7c0 .525-.157 1.034-.445 1.465.183.302.289.656.289 1.035v.106a3.596 3.596 0 0 0 .06-5.15A3.6 3.6 0 1 0 5.5 9.59V9.5c0-.384.108-.743.296-1.047A2.64 2.64 0 1 1 10.64 7z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 7a1 1 0 1 1-2 0 1 1 0 0 1 2 0z" /> < title > { title } < / title > < / svg >
    }
}
