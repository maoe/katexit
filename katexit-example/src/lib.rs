#[cfg_attr(doc, katexit::katexit)]
//! Can we write $\LaTeX$ expressions at the module-level doc?

#[cfg_attr(doc, katexit::katexit)]
/// We can write $\LaTeX$ expressions
///
/// Display style
/// -------------
///
/// $$
/// c = \\pm\\sqrt{a^2 + b^2}
/// $$
pub fn my_func() {}
