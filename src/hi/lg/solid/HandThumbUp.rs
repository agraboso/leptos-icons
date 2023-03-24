#[cfg(feature = "HiLgSolidHandThumbUp")]
use leptos::*;
#[cfg(feature = "HiLgSolidHandThumbUp")]
///This icon requires the feature `HiLgSolidHandThumbUp` to be enabled.
#[component]
pub fn HandThumbUp(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.49281 18.75C7.06823 18.75 6.67296 18.5135 6.51759 18.1184C6.18349 17.2687 6 16.3433 6 15.375C6 13.6259 6.59874 12.0167 7.60244 10.741C7.75335 10.5493 7.97456 10.4321 8.20214 10.344C8.67496 10.1609 9.09254 9.82968 9.4141 9.41967C10.1873 8.43384 11.1617 7.6134 12.2755 7.02021C12.9977 6.63563 13.6243 6.06428 13.9281 5.30464C14.1408 4.7731 14.25 4.20587 14.25 3.63338V3C14.25 2.58579 14.5858 2.25 15 2.25C16.2426 2.25 17.25 3.25736 17.25 4.5C17.25 5.65163 16.9904 6.74263 16.5266 7.71771C16.261 8.27604 16.6336 9 17.2519 9H20.3777C21.4044 9 22.3233 9.69399 22.432 10.7149C22.4769 11.1371 22.5 11.5658 22.5 12C22.5 14.8476 21.5081 17.4636 19.851 19.5212C19.4634 20.0025 18.8642 20.25 18.2462 20.25H14.2302C13.7466 20.25 13.2661 20.172 12.8072 20.0191L9.69278 18.9809C9.23393 18.828 8.75342 18.75 8.26975 18.75H7.49281Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.33149 10.9771C1.79481 12.3389 1.5 13.8225 1.5 15.375C1.5 16.5951 1.68208 17.7726 2.02056 18.882C2.27991 19.732 3.10418 20.25 3.99289 20.25H4.90067C5.3462 20.25 5.62137 19.7517 5.42423 19.3522C4.83248 18.1529 4.5 16.8028 4.5 15.375C4.5 13.6668 4.97588 12.0698 5.8023 10.7093C6.0473 10.3059 5.77404 9.75 5.30212 9.75H4.24936C3.41733 9.75 2.63655 10.203 2.33149 10.9771Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
