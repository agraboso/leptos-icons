#[cfg(feature = "HiMdSolidUserGroup")]
use leptos::*;
#[cfg(feature = "HiMdSolidUserGroup")]
///This icon requires the feature `HiMdSolidUserGroup` to be enabled.
#[component]
pub fn UserGroup(
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
        "http://www.w3.org/2000/svg" d =
        "M10 9C11.6569 9 13 7.65685 13 6C13 4.34315 11.6569 3 10 3C8.34315 3 7 4.34315 7 6C7 7.65685 8.34315 9 10 9Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 8C6 9.10457 5.10457 10 4 10C2.89543 10 2 9.10457 2 8C2 6.89543 2.89543 6 4 6C5.10457 6 6 6.89543 6 8Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.49064 15.3257C1.32107 15.2271 1.19021 15.0718 1.13247 14.8843C1.04636 14.6048 1 14.3078 1 14C1 12.3431 2.34315 11 4 11C4.522 11 5.01287 11.1333 5.4404 11.3678C4.39329 12.3989 3.69414 13.7825 3.53478 15.3267C3.5118 15.5494 3.52139 15.7692 3.55996 15.9809C2.81061 15.9156 2.10861 15.6849 1.49064 15.3257Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.4405 15.9809C17.1897 15.9155 17.8915 15.6849 18.5094 15.3257C18.6789 15.2271 18.8098 15.0718 18.8675 14.8843C18.9536 14.6048 19 14.3078 19 14C19 12.3431 17.6569 11 16 11C15.4781 11 14.9873 11.1333 14.5599 11.3676C15.6071 12.3987 16.3063 13.7824 16.4656 15.3267C16.4886 15.5494 16.479 15.7692 16.4405 15.9809Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 8C18 9.10457 17.1046 10 16 10C14.8954 10 14 9.10457 14 8C14 6.89543 14.8954 6 16 6C17.1046 6 18 6.89543 18 8Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.30383 16.1909C5.10473 16.0106 4.99922 15.7478 5.02679 15.4807C5.28657 12.9633 7.41408 11 10.0001 11C12.5862 11 14.7137 12.9633 14.9735 15.4807C15.0011 15.7478 14.8956 16.0107 14.6965 16.1909C13.4545 17.3152 11.8073 18 10.0001 18C8.19298 18 6.54576 17.3152 5.30383 16.1909Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
