//! The document model.

mod content;
mod element;
mod introspect;
mod realize;
mod styles;

pub use self::content::*;
pub use self::element::*;
pub use self::introspect::*;
pub use self::realize::*;
pub use self::styles::*;

pub use typst_macros::element;

use comemo::{Constraint, Track, Tracked, TrackedMut};

use crate::diag::SourceResult;
use crate::doc::Document;
use crate::eval::Route;
use crate::eval::Scopes;
use crate::eval::Tracer;
use crate::eval::Vm;
use crate::syntax::SourceId;
use crate::World;

/// Typeset content into a fully layouted document.
#[comemo::memoize]
pub fn typeset(
    world: Tracked<dyn World>,
    mut tracer: TrackedMut<Tracer>,
    content: &Content,
) -> SourceResult<Document> {
    let library = world.library();
    let styles = StyleChain::new(&library.styles);

    let mut document;
    let mut iter = 0;
    let mut introspector = Introspector::new(&[]);

    // Relayout until all introspections stabilize.
    // If that doesn't happen within five attempts, we give up.
    loop {
        let constraint = Constraint::new();
        let mut provider = StabilityProvider::new();
        let route = Route::default();
        let id = SourceId::detached();
        let scopes = Scopes::new(Some(library));
        let mut vm = Vm::new(
            world,
            TrackedMut::reborrow_mut(&mut tracer),
            provider.track_mut(),
            introspector.track_with(&constraint),
            route.track(),
            id,
            scopes,
        );

        document = (library.items.layout)(&mut vm, content, styles)?;
        iter += 1;

        introspector = Introspector::new(&document.pages);

        if iter >= 5 || introspector.valid(&constraint) {
            break;
        }
    }

    Ok(document)
}
