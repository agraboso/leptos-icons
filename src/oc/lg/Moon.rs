#[cfg(feature = "OcLgMoon")]
use leptos::*;
#[cfg(feature = "OcLgMoon")]
///This icon requires the feature `OcLgMoon` to be enabled.
#[component]
pub fn Moon(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.768 3.96v.001l-.002-.005a9.08 9.08 0 0 0-.218-.779c-.13-.394.21-.8.602-.67.29.096.575.205.855.328l.01.005A10.002 10.002 0 0 1 12 22a10.002 10.002 0 0 1-9.162-5.985l-.004-.01a9.722 9.722 0 0 1-.329-.855c-.13-.392.277-.732.67-.602.257.084.517.157.78.218l.004.002A9 9 0 0 0 14.999 6a9.09 9.09 0 0 0-.231-2.04ZM16.5 6c0 5.799-4.701 10.5-10.5 10.5-.426 0-.847-.026-1.26-.075A8.5 8.5 0 1 0 16.425 4.74c.05.413.075.833.075 1.259Z"
        /> < title > { title } < / title > < / svg >
    }
}
