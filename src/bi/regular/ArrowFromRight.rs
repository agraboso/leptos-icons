#[cfg(feature = "BiRegularArrowFromRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArrowFromRight")]
/// *This icon requires the feature* `BiRegularArrowFromRight` *to be enabled*.
#[component]
pub fn ArrowFromRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 6h2v12h-2zm-2 5H7.414l4.293-4.293-1.414-1.414L3.586 12l6.707 6.707 1.414-1.414L7.414 13H16z" /></svg>
   }
}