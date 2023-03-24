#[cfg(feature = "SiHootsuite")]
use leptos::*;
#[cfg(feature = "SiHootsuite")]
///This icon requires the feature `SiHootsuite` to be enabled.
#[component]
pub fn Hootsuite(
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
        "M12.002 0h.023c1.311.004 2.603.322 3.766.928C16.948.332 18.23.022 19.532.022h.676V24l-.656-.002C15.369 24 11.356 22.336 8.4 19.373 5.43 16.43 3.77 12.414 3.791 8.23V.021h.677c1.301 0 2.586.311 3.741.906C9.381.318 10.682 0 12.002 0zm0 .654c-1.381 0-2.676.373-3.791 1.021-1.138-.655-2.428-1.001-3.742-1h-.022V8.23c-.025 8.35 6.764 15.09 15.107 15.113V.675h-.022c-1.313-.001-2.604.343-3.743.999-1.144-.666-2.443-1.018-3.766-1.02h-.021zm3.252 2.754c1.79.002 3.238 1.453 3.237 3.242-.003 1.791-1.454 3.238-3.244 3.236-.616 0-1.22-.176-1.739-.508l-1.516 1.508-1.507-1.516c-1.514.952-3.515.495-4.465-1.02-.952-1.516-.495-3.516 1.021-4.467s3.516-.494 4.467 1.022c.273.437.44.933.483 1.446l.016-.02.015.018c.154-1.667 1.556-2.945 3.232-2.941zM8.76 8.789c1.192.006 2.163-.959 2.168-2.15.001-.219-.031-.436-.096-.644-.243.544-.882.788-1.426.546-.545-.244-.79-.883-.546-1.428.109-.243.304-.437.548-.547-1.137-.355-2.347.276-2.705 1.414-.066.207-.099.424-.1.642-.003 1.192.96 2.163 2.153 2.167h.004zm6.478.019c1.193.003 2.163-.962 2.166-2.155s-.963-2.162-2.155-2.164c-.216-.002-.431.03-.638.094.545.244.789.883.547 1.428-.244.543-.883.787-1.428.545-.245-.109-.439-.307-.549-.553-.355 1.139.279 2.352 1.417 2.707.209.063.423.097.64.098z"
        /> < title > { title } < / title > < / svg >
    }
}
