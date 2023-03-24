#[cfg(feature = "HiLgSolidChatBubbleLeftEllipsis")]
use leptos::*;
#[cfg(feature = "HiLgSolidChatBubbleLeftEllipsis")]
///This icon requires the feature `HiLgSolidChatBubbleLeftEllipsis` to be enabled.
#[component]
pub fn ChatBubbleLeftEllipsis(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M12.0003 2.25C9.57113 2.25 7.18302 2.42773 4.84836 2.771C2.87004 3.06188 1.5 4.79455 1.5 6.74064V12.7593C1.5 14.7054 2.87004 16.4381 4.84836 16.729C5.72491 16.8579 6.60901 16.9634 7.5 17.045V21C7.5 21.3033 7.68273 21.5768 7.96299 21.6929C8.24324 21.809 8.56583 21.7448 8.78033 21.5303L12.9638 17.3468C13.0318 17.2788 13.1266 17.2374 13.2302 17.2348C15.2361 17.1851 17.2123 17.0142 19.1516 16.7291C21.1299 16.4382 22.5 14.7056 22.5 12.7595V6.74056C22.5 4.79445 21.1299 3.06177 19.1516 2.77091C16.8171 2.4277 14.4292 2.25 12.0003 2.25ZM8.25 8.625C7.62868 8.625 7.125 9.12868 7.125 9.75C7.125 10.3713 7.62868 10.875 8.25 10.875C8.87132 10.875 9.375 10.3713 9.375 9.75C9.375 9.12868 8.87132 8.625 8.25 8.625ZM10.875 9.75C10.875 9.12868 11.3787 8.625 12 8.625C12.6213 8.625 13.125 9.12868 13.125 9.75C13.125 10.3713 12.6213 10.875 12 10.875C11.3787 10.875 10.875 10.3713 10.875 9.75ZM15.75 8.625C15.1287 8.625 14.625 9.12868 14.625 9.75C14.625 10.3713 15.1287 10.875 15.75 10.875C16.3713 10.875 16.875 10.3713 16.875 9.75C16.875 9.12868 16.3713 8.625 15.75 8.625Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
