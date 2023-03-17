#[cfg(feature = "BiSolidClinic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidClinic")]
/// *This icon requires the feature* `BiSolidClinic` *to be enabled*.
#[component]
pub fn Clinic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21.743 12.331-9-10c-.379-.422-1.107-.422-1.486 0l-9 10a.998.998 0 0 0-.17 1.076c.16.361.518.593.913.593h2v7a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-7h2a.998.998 0 0 0 .743-1.669zM16 15h-3v3h-2v-3H8v-2h3v-3h2v3h3v2z" /></svg>
   }
}