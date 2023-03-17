#[cfg(feature = "RiMediaFillClapperboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaFillClapperboard")]
/// *This icon requires the feature* `RiMediaFillClapperboard` *to be enabled*.
#[component]
pub fn Clapperboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M17.998 7l2.31-4h.7c.548 0 .992.445.992.993v16.014a1 1 0 0 1-.992.993H2.992A.993.993 0 0 1 2 20.007V3.993A1 1 0 0 1 2.992 3h3.006l-2.31 4h2.31l2.31-4h3.69l-2.31 4h2.31l2.31-4h3.69l-2.31 4h2.31z" /></g></svg>
   }
}