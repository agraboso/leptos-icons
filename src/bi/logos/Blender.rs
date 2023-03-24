#[cfg(feature = "BiLogosBlender")]
use leptos::*;
#[cfg(feature = "BiLogosBlender")]
///This icon requires the feature `BiLogosBlender` to be enabled.
#[component]
pub fn Blender(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.427 13.011c.037-.667.363-1.254.856-1.671a2.855 2.855 0 0 1 1.844-.66c.71 0 1.36.25 1.845.66.492.417.819 1.005.856 1.671.038.686-.237 1.323-.721 1.795a2.829 2.829 0 0 1-1.979.782 2.83 2.83 0 0 1-1.981-.782c-.483-.472-.759-1.109-.72-1.795z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.124 14.361c.005.26.089.767.213 1.164a6.156 6.156 0 0 0 1.328 2.299 6.833 6.833 0 0 0 2.323 1.667 7.465 7.465 0 0 0 3.05.635 7.495 7.495 0 0 0 3.051-.645 6.913 6.913 0 0 0 2.321-1.675 6.196 6.196 0 0 0 1.326-2.303 5.76 5.76 0 0 0 .25-1.285 5.942 5.942 0 0 0-.888-3.594 6.496 6.496 0 0 0-1.545-1.703l.001-.001-6.249-4.799-.016-.014c-.411-.314-1.101-.313-1.551.002-.457.319-.508.846-.104 1.18l-.001.001 2.606 2.121-7.943.009h-.012c-.656 0-1.287.432-1.412.976-.128.555.318 1.015 1.001 1.017l-.001.003 4.027-.008-7.188 5.516-.027.021c-.677.519-.896 1.382-.47 1.929.434.556 1.354.556 2.04.002l3.922-3.209c.001 0-.056.433-.052.694zm10.078 1.45c-.808.824-1.938 1.291-3.163 1.293-1.226.002-2.356-.461-3.165-1.283a3.739 3.739 0 0 1-.864-1.352 3.503 3.503 0 0 1-.199-1.511c.044-.505.193-.987.434-1.422.236-.429.562-.815.962-1.144a4.477 4.477 0 0 1 2.832-.988 4.478 4.478 0 0 1 2.832.98c.399.326.725.711.961 1.139.24.436.39.916.434 1.421a3.52 3.52 0 0 1-.198 1.511 3.804 3.804 0 0 1-.866 1.356z"
        /> < title > { title } < / title > < / svg >
    }
}
