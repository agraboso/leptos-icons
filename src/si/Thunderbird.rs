#[cfg(feature = "SiThunderbird")]
use leptos::*;
#[cfg(feature = "SiThunderbird")]
///This icon requires the feature `SiThunderbird` to be enabled.
#[component]
pub fn Thunderbird(
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
        "M23.7296 10.23c-.225-1.1996-.6936-2.3617-1.3308-3.4113-.1806-.3233-.4172-.6295-.6186-.9185-1.3495-1.8931-2.418-2.5304-2.6241-2.6429-.1874-.1312-.3749-.2624-.5623-.3748-1.7432-1.0872-2.8678-1.2371-2.8678-1.2371-.806-.3374-1.5932-.5248-2.1743-.6186-1.2558-.15-2.3055-.1312-3.2239 0 .15-.2624.2812-.356.3186-.3748.0232 0 .0166 0 0 0-.4498.0375-.9934.3374-1.4807.6373C9.503.6333 10.2153.2583 10.309.202h.0187C9.7655.1646 8.1723.5208 6.804 2.0391 2.549 3.8196 0 7.7183 0 11.973c0 3.224 1.2558 5.5482 2.6241 7.0852 1.2933 1.4058 2.7179 2.1555 3.3926 2.3617.0708.0279.225.0375.225.0375.0187-.0562-.8622-1.462-1.2559-3.149.7873.806 1.7245 1.4246 2.7741 1.537.1125.0188-.3749-.581-.8997-1.3682l.7123.2436 7.8161 2.6054c-.7685.7872-1.7994 1.5557-3.2052 2.2305 0 0 3.149-.2624 4.7047-2.0056-.6373 1.462-2.3805 2.2493-2.3805 2.2493 1.2746-.1875 4.6485-1.1809 6.7853-3.8612 2.4367-3.0365 3.1677-5.8856 2.4367-9.7093zm-6.223 7.1039c-.2436 1.1808-.7497 2.493-1.893 3.7487L1.7993 16.4904c-.3936-1.2558-.581-2.7178-.4686-4.4235.2437.4499.9747.8247 1.4245.8435-1.0496-2.343-.6373-3.955.15-5.0233a635.68 635.68 0 0 1 1.0309 1.5182v-.1125c.0187-.15.0375-.2624.075-.3748-.3562-.5249-.656-.9747-.881-1.2934.4311-.4873.9184-.8434 1.3496-1.0871.0187.2062.0562.3936.1124.6185.075.3187.0937.7873.0562 1.1622v-.0188c0 .075-.0187.15-.0374.225-.0188.075-.225.281-.2812.8247-.0562.581.15.8997.3561 1.1433.2437-.2249.6936-.7872 1.5183-1.1059.8247-.3373 1.3683-.8247 2.418-1.3495.6185-.3186 1.2745-.2812 2.0992-.2437 1.4808.2624 3.4301.7873 5.2483.881.4123.9747.581 2.3804.581 2.418 0 .1311.0188.2436.0188.356-3.224 1.2184-7.0102 2.6054-8.6034 3.0178-.1312.0375-1.5183-1.968-2.8116-3.88-.0187.0375-.0562.0563-.075.075-.0374.0375-.075.075-.0937.1125h-.1124c1.387 2.043 2.8678 4.161 3.0177 4.161 1.4058-.506 5.6044-1.9493 8.6596-2.9989-.0187 2.3055-.581 3.1677-.581 3.1677s.7685-.3 1.5557-1.0871c0 .6372-.1687 2.2867-1.4995 3.8799 0 0 .6935-.1874 1.4807-.5623Z"
        /> < title > { title } < / title > < / svg >
    }
}
