#[cfg(feature = "VsIssueReopened")]
use leptos::*;
#[cfg(feature = "VsIssueReopened")]
///This icon requires the feature `VsIssueReopened` to be enabled.
#[component]
pub fn IssueReopened(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M5.28 5.65556L2 7.00556L1.34 6.74556L0 3.50556L0.92 3.12556L1.73 5.07556C2.27376 3.71475 3.2627 2.57844 4.53544 1.85207C5.80817 1.1257 7.28953 0.852174 8.73774 1.07614C10.1859 1.3001 11.5155 2.00832 12.5093 3.08521C13.5032 4.1621 14.1027 5.54407 14.21 7.00556H13.21C13.0956 5.75683 12.5564 4.58511 11.6824 3.68594C10.8083 2.78677 9.65237 2.21456 8.40739 2.06478C7.1624 1.91501 5.90371 2.19674 4.84137 2.86297C3.77903 3.52919 2.97731 4.53959 2.57 5.72556L4.89 4.72556L5.28 5.65556ZM14.14 8.33562L15.48 11.5656L14.56 12.0056L13.74 10.0056C13.1919 11.3718 12.1958 12.511 10.9149 13.2364C9.63412 13.9618 8.14476 14.2302 6.69127 13.9977C5.23779 13.7651 3.90654 13.0454 2.91599 11.9566C1.92544 10.8678 1.33445 9.47455 1.24001 8.00562H2.24001V7.50562C2.24281 8.79308 2.69801 10.0386 3.52602 11.0245C4.35404 12.0104 5.5022 12.6739 6.76983 12.899C8.03745 13.1242 9.34388 12.8967 10.4608 12.2563C11.5777 11.6159 12.434 10.6033 12.88 9.39562L10.63 10.3256L10.24 9.40561L13.49 8.05562L14.14 8.33562Z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "7.74001" cy = "7.53955" r =
        "1" /> < title > { title } < / title > < / svg >
    }
}
