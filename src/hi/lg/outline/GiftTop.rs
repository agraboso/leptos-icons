#[cfg(feature = "HiLgOutlineGiftTop")]
use leptos::*;
#[cfg(feature = "HiLgOutlineGiftTop")]
///This icon requires the feature `HiLgOutlineGiftTop` to be enabled.
#[component]
pub fn GiftTop(
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
        "M12 3.75V20.25M2.25 12H21.75M6.375 17.25C9.06739 17.25 11.25 15.0674 11.25 12.375V12M17.625 17.25C14.9326 17.25 12.75 15.0674 12.75 12.375V12M3.75 20.25H20.25C21.0784 20.25 21.75 19.5784 21.75 18.75V5.25C21.75 4.42157 21.0784 3.75 20.25 3.75H3.75C2.92157 3.75 2.25 4.42157 2.25 5.25V18.75C2.25 19.5784 2.92157 20.25 3.75 20.25ZM16.3713 10.8107C14.9623 12.2197 12.1286 11.8714 12.1286 11.8714C12.1286 11.8714 11.7803 9.03772 13.1893 7.62871C14.068 6.75003 15.4926 6.75003 16.3713 7.62871C17.2499 8.50739 17.2499 9.93201 16.3713 10.8107ZM10.773 7.62874C12.182 9.03775 11.8336 11.8714 11.8336 11.8714C11.8336 11.8714 9 12.2197 7.59099 10.8107C6.71231 9.93204 6.71231 8.50742 7.59099 7.62874C8.46967 6.75006 9.89429 6.75006 10.773 7.62874Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
