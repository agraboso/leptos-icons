#[cfg(feature = "HiMdSolidPaperClip")]
use leptos::*;
#[cfg(feature = "HiMdSolidPaperClip")]
///This icon requires the feature `HiMdSolidPaperClip` to be enabled.
#[component]
pub fn PaperClip(
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
        "M15.6213 4.37868C14.4497 3.20711 12.5503 3.20711 11.3787 4.37868L4.37868 11.3787C3.20711 12.5503 3.20711 14.4497 4.37868 15.6213C5.54995 16.7926 7.44878 16.7929 8.62042 15.6222C8.62072 15.6219 8.62102 15.6216 8.62132 15.6213L9.11792 15.1214C9.40985 14.8276 9.88472 14.826 10.1786 15.1179C10.4724 15.4098 10.474 15.8847 10.1821 16.1786L9.68373 16.6802L9.68198 16.682C7.92462 18.4393 5.07538 18.4393 3.31802 16.682C1.56066 14.9246 1.56066 12.0754 3.31802 10.318L10.318 3.31802C12.0754 1.56066 14.9246 1.56066 16.682 3.31802C18.438 5.07407 18.4393 7.92038 16.6859 9.67806L13.2312 13.2312C12.2061 14.2564 10.544 14.2563 9.51885 13.2312C8.49372 12.206 8.49372 10.544 9.51885 9.51886L12.9697 6.06804C13.2626 5.77515 13.7374 5.77515 14.0303 6.06804C14.3232 6.36094 14.3232 6.83581 14.0303 7.1287L10.5795 10.5795C10.1402 11.0189 10.1402 11.7312 10.5795 12.1705C11.0178 12.6088 11.7276 12.6099 12.1672 12.1738L15.6213 8.62127C16.7928 7.4497 16.7929 5.55025 15.6213 4.37868Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
