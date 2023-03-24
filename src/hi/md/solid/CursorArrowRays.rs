#[cfg(feature = "HiMdSolidCursorArrowRays")]
use leptos::*;
#[cfg(feature = "HiMdSolidCursorArrowRays")]
///This icon requires the feature `HiMdSolidCursorArrowRays` to be enabled.
#[component]
pub fn CursorArrowRays(
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
        "M10 1C10.4142 1 10.75 1.33579 10.75 1.75V3.25C10.75 3.66421 10.4142 4 10 4C9.58579 4 9.25 3.66421 9.25 3.25V1.75C9.25 1.33579 9.58579 1 10 1ZM5.05025 3.05025C5.34315 2.75736 5.81802 2.75736 6.11091 3.05025L7.17157 4.11091C7.46447 4.40381 7.46447 4.87868 7.17157 5.17157C6.87868 5.46447 6.40381 5.46447 6.11091 5.17157L5.05025 4.11091C4.75736 3.81802 4.75736 3.34315 5.05025 3.05025ZM14.9497 3.05025C15.2426 3.34315 15.2426 3.81802 14.9497 4.11091L13.8891 5.17157C13.5962 5.46447 13.1213 5.46447 12.8284 5.17157C12.5355 4.87868 12.5355 4.40381 12.8284 4.11091L13.8891 3.05025C14.182 2.75736 14.6569 2.75736 14.9497 3.05025ZM3 8C3 7.58579 3.33579 7.25 3.75 7.25H5.25C5.66421 7.25 6 7.58579 6 8C6 8.41421 5.66421 8.75 5.25 8.75H3.75C3.33579 8.75 3 8.41421 3 8ZM14 8C14 7.58579 14.3358 7.25 14.75 7.25H16.25C16.6642 7.25 17 7.58579 17 8C17 8.41421 16.6642 8.75 16.25 8.75H14.75C14.3358 8.75 14 8.41421 14 8ZM7.17157 10.8284C7.46447 11.1213 7.46447 11.5962 7.17157 11.8891L6.11091 12.9497C5.81802 13.2426 5.34315 13.2426 5.05025 12.9497C4.75736 12.6569 4.75736 12.182 5.05025 11.8891L6.11091 10.8284C6.40381 10.5355 6.87868 10.5355 7.17157 10.8284ZM10.7657 7.51062C10.5871 7.24492 10.2596 7.12184 9.95029 7.20417C9.64094 7.2865 9.41795 7.5561 9.3951 7.8754L8.90409 14.7363C8.88303 15.0306 9.0365 15.3099 9.29622 15.4499C9.55594 15.59 9.87365 15.5647 10.108 15.3854L11.1508 14.5873L12.1363 18.2653C12.2435 18.6654 12.6548 18.9028 13.0549 18.7956C13.455 18.6884 13.6924 18.2771 13.5852 17.877L12.6083 14.2312L13.9005 14.4349C14.1951 14.4814 14.4893 14.3489 14.6497 14.0974C14.81 13.846 14.8062 13.5233 14.6398 13.2758L10.7657 7.51062Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
