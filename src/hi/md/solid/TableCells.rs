#[cfg(feature = "HiMdSolidTableCells")]
use leptos::*;
#[cfg(feature = "HiMdSolidTableCells")]
///This icon requires the feature `HiMdSolidTableCells` to be enabled.
#[component]
pub fn TableCells(
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
        "M0.98999 5.23999C0.98999 3.99735 2.00736 3 3.25 3H16.75C17.9926 3 19 4.00736 19 5.25L19.01 14.75C19.01 15.9926 18.0027 17 16.76 17C12.9267 17 7.09334 17 3.26001 17C2.01737 17 1 15.9826 1 14.74L0.98999 5.23999ZM9.25 14.76V14.135C9.25 13.7208 8.91421 13.385 8.5 13.385H3.25C2.83579 13.385 2.5 13.7208 2.5 14.135V14.75C2.5 15.1642 2.83579 15.5 3.25 15.5H8.62283C8.97861 15.4414 9.25 15.1324 9.25 14.76ZM10.75 14.76C10.75 15.1324 11.0214 15.4414 11.3772 15.5H16.75C17.1642 15.5 17.5 15.1642 17.5 14.75V14.135C17.5 13.7208 17.1642 13.385 16.75 13.385H11.5C11.0858 13.385 10.75 13.7208 10.75 14.135V14.76ZM17.5 11.13V10.505C17.5 10.0908 17.1642 9.755 16.75 9.755H11.5C11.0858 9.755 10.75 10.0908 10.75 10.505V11.13C10.75 11.5442 11.0858 11.88 11.5 11.88H16.75C17.1642 11.88 17.5 11.5442 17.5 11.13ZM9.25 11.13V10.505C9.25 10.0908 8.91421 9.755 8.5 9.755H3.25C2.83579 9.755 2.5 10.0908 2.5 10.505V11.13C2.5 11.5442 2.83579 11.88 3.25 11.88H8.5C8.91421 11.88 9.25 11.5442 9.25 11.13ZM17.5 7.5V6.875C17.5 6.46079 17.1642 6.125 16.75 6.125H11.5C11.0858 6.125 10.75 6.46079 10.75 6.875V7.5C10.75 7.91421 11.0858 8.25 11.5 8.25H16.75C17.1642 8.25 17.5 7.91421 17.5 7.5ZM9.25 7.5V6.875C9.25 6.46079 8.91421 6.125 8.5 6.125H3.25C2.83579 6.125 2.5 6.46079 2.5 6.875V7.5C2.5 7.91421 2.83579 8.25 3.25 8.25H8.5C8.91421 8.25 9.25 7.91421 9.25 7.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
