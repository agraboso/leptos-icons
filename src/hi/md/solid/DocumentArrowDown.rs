#[cfg(feature = "HiMdSolidDocumentArrowDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidDocumentArrowDown")]
///This icon requires the feature `HiMdSolidDocumentArrowDown` to be enabled.
#[component]
pub fn DocumentArrowDown(
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
        "M4.5 2C3.67157 2 3 2.67157 3 3.5V16.5C3 17.3284 3.67157 18 4.5 18H15.5C16.3284 18 17 17.3284 17 16.5V7.62132C17 7.2235 16.842 6.84197 16.5607 6.56066L12.4393 2.43934C12.158 2.15804 11.7765 2 11.3787 2H4.5ZM9.25 8.75C9.25 8.33579 9.58579 8 10 8C10.4142 8 10.75 8.33579 10.75 8.75V11.2955L11.6925 10.2483C11.9696 9.94039 12.4438 9.91543 12.7517 10.1925C13.0596 10.4696 13.0846 10.9438 12.8075 11.2517L10.5575 13.7517C10.4152 13.9098 10.2126 14 10 14C9.78739 14 9.58476 13.9098 9.44253 13.7517L7.19253 11.2517C6.91544 10.9438 6.94039 10.4696 7.24828 10.1925C7.55616 9.91544 8.03038 9.94039 8.30747 10.2483L9.25 11.2955V8.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
