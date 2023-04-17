use crate::prelude::*;

/// Measure the layouted size of content.
///
/// The `measure` function lets you determine the layouted size of content.
/// The same content can have a different size depending on the styles that
/// are active when it is layouted. For example, in the example below
/// `[#content]` is of course bigger when we increase the font size.
///
/// ```example
/// #let content = [Hello!]
/// #content
/// #set text(14pt)
/// #content
/// ```
///
/// To do a meaningful measurement, you therefore first need to retrieve the
/// active styles with the [`style`]($func/style) function. You can then pass
/// them to the `measure` function.
///
/// ```example
/// #let thing(body) = style(styles => {
///   let size = measure(body, styles)
///   [Width of "#body" is #size.width]
/// })
///
/// #thing[Hey] \
/// #thing[Welcome]
/// ```
///
/// The measure function returns a dictionary with the entries `width` and
/// `height`, both of type [`length`]($type/length).
///
/// Display: Measure
/// Category: layout
/// Returns: dictionary
#[func]
pub fn measure(
    /// The content whose size to measure.
    content: Content,
    /// The styles with which to layout the content.
    styles: Styles,
) -> Value {
    let pod = Regions::one(Axes::splat(Abs::inf()), Axes::splat(false));
    let styles = StyleChain::new(&styles);
    let frame = content.measure(vm, styles, pod)?.into_frame();
    let Size { x, y } = frame.size();
    dict! { "width" => x, "height" => y }.into()
}
