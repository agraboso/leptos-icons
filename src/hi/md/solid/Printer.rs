#[cfg(feature = "HiMdSolidPrinter")]
use leptos::*;
#[cfg(feature = "HiMdSolidPrinter")]
///This icon requires the feature `HiMdSolidPrinter` to be enabled.
#[component]
pub fn Printer(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M5 2.75C5 1.7835 5.7835 1 6.75 1H13.25C14.2165 1 15 1.7835 15 2.75V6.25C15 6.26733 14.9997 6.28459 14.9992 6.3018C15.3765 6.34767 15.7522 6.39867 16.1263 6.45472C17.2246 6.6193 18 7.57138 18 8.65297V12.75C18 13.9926 16.9926 15 15.75 15H15.5086L15.8138 16.9839C15.9769 18.044 15.1567 19 14.0842 19H5.91541C4.84287 19 4.02268 18.044 4.18576 16.9839L4.49098 15H4.25C3.00736 15 2 13.9926 2 12.75V8.65298C2 7.57138 2.77538 6.6193 3.87368 6.45472C4.24776 6.39867 4.62347 6.34767 5.00075 6.3018C5.00025 6.28459 5 6.26733 5 6.25V2.75ZM13.5 6.14734C12.3461 6.04977 11.1788 6 10 6C8.82119 6 7.65386 6.04977 6.5 6.14734V2.75C6.5 2.61193 6.61193 2.5 6.75 2.5H13.25C13.3881 2.5 13.5 2.61193 13.5 2.75V6.14734ZM6.60772 12.5C6.48433 12.5 6.37939 12.59 6.36063 12.712L5.66832 17.212C5.64502 17.3634 5.76219 17.5 5.91541 17.5H14.0842C14.2374 17.5 14.3546 17.3634 14.3313 17.212L13.639 12.712C13.6202 12.59 13.5153 12.5 13.3919 12.5H6.60772Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
