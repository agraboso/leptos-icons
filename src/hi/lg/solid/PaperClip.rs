#[cfg(feature = "HiLgSolidPaperClip")]
use leptos::*;
#[cfg(feature = "HiLgSolidPaperClip")]
///This icon requires the feature `HiLgSolidPaperClip` to be enabled.
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M18.9696 3.65901C18.0909 2.78033 16.6663 2.78033 15.7876 3.65901L4.84835 14.5983C3.38388 16.0628 3.38388 18.4372 4.84835 19.9016C6.31282 21.3661 8.68718 21.3661 10.1517 19.9016L17.8446 12.2087C18.1375 11.9158 18.6124 11.9158 18.9053 12.2087C19.1982 12.5015 19.1982 12.9764 18.9053 13.2693L11.2123 20.9623C9.16206 23.0126 5.83794 23.0126 3.78769 20.9623C1.73744 18.9121 1.73744 15.5879 3.78769 13.5377L14.7269 2.59835C16.1914 1.13388 18.5658 1.13388 20.0302 2.59835C21.4947 4.06281 21.4947 6.43718 20.0302 7.90165L9.09735 18.8346C9.09452 18.8374 9.09167 18.8403 9.08878 18.8432L9.08191 18.85L9.07971 18.8522L9.07747 18.8544C8.19765 19.7196 6.78319 19.7152 5.90901 18.841C5.03033 17.9623 5.03033 16.5377 5.90901 15.659L13.7195 7.84833C14.0124 7.55543 14.4873 7.55542 14.7802 7.84831C15.0731 8.1412 15.0731 8.61608 14.7802 8.90897L6.96968 16.7197C6.67678 17.0125 6.67678 17.4874 6.96967 17.7803C7.26041 18.0711 7.73056 18.0732 8.02383 17.7867L18.9696 6.84099C19.8483 5.96231 19.8483 4.53769 18.9696 3.65901Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
