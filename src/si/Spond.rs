#[cfg(feature = "SiSpond")]
use leptos::*;
#[cfg(feature = "SiSpond")]
///This icon requires the feature `SiSpond` to be enabled.
#[component]
pub fn Spond(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm-1.403 2.281a3.767 3.767 0 00-.17 2.847c.61 1.79 2.336 2.772 4.069 3.213 2.633.672 4.715 1.388 5.892 2.502 1.037.982 1.435 2.416.803 4.618-.17.59-.486 1.124-.802 1.643-.125-.706-.424-1.411-.924-2.094-3.269-4.462-10.438-3.57-13.174-7.307-.803-1.096-.747-2.236.092-3.288.979-1.226 2.69-1.917 4.214-2.134zm3.163.11c.138-.01.281.002.43.036a9.835 9.835 0 017.076 6.318c-1.514-1.132-3.655-1.86-6.233-2.517-1.528-.39-2.3-1.087-2.542-1.798-.326-.956.308-1.98 1.27-2.04zM3.611 6.895c.125.706.424 1.412.924 2.094 3.269 4.462 10.438 3.57 13.174 7.307.803 1.095.747 2.236-.092 3.288-.979 1.226-2.69 1.916-4.214 2.133.427-.89.489-1.91.17-2.846-.61-1.79-2.336-2.772-4.069-3.213-2.633-.672-4.715-1.388-5.892-2.502-1.037-.982-1.435-2.416-.803-4.618.17-.59.486-1.124.802-1.643zm-.877 8.36c1.514 1.13 3.655 1.858 6.233 2.516 1.528.39 2.3 1.087 2.542 1.798.336.985-.347 2.042-1.357 2.042-.11 0-.225-.012-.342-.039a9.835 9.835 0 01-7.076-6.318z"
        /> < title > { title } < / title > < / svg >
    }
}
