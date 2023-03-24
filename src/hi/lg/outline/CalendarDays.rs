#[cfg(feature = "HiLgOutlineCalendarDays")]
use leptos::*;
#[cfg(feature = "HiLgOutlineCalendarDays")]
///This icon requires the feature `HiLgOutlineCalendarDays` to be enabled.
#[component]
pub fn CalendarDays(
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
        "M6.75 3V5.25M17.25 3V5.25M3 18.75V7.5C3 6.25736 4.00736 5.25 5.25 5.25H18.75C19.9926 5.25 21 6.25736 21 7.5V18.75M3 18.75C3 19.9926 4.00736 21 5.25 21H18.75C19.9926 21 21 19.9926 21 18.75M3 18.75V11.25C3 10.0074 4.00736 9 5.25 9H18.75C19.9926 9 21 10.0074 21 11.25V18.75M12 12.75H12.0075V12.7575H12V12.75ZM12 15H12.0075V15.0075H12V15ZM12 17.25H12.0075V17.2575H12V17.25ZM9.75 15H9.7575V15.0075H9.75V15ZM9.75 17.25H9.7575V17.2575H9.75V17.25ZM7.5 15H7.5075V15.0075H7.5V15ZM7.5 17.25H7.5075V17.2575H7.5V17.25ZM14.25 12.75H14.2575V12.7575H14.25V12.75ZM14.25 15H14.2575V15.0075H14.25V15ZM14.25 17.25H14.2575V17.2575H14.25V17.25ZM16.5 12.75H16.5075V12.7575H16.5V12.75ZM16.5 15H16.5075V15.0075H16.5V15Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
