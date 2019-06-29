use cairo;
use cairo::{PatternTrait, Rectangle};
use markup5ever::local_name;
use std::cell::{Cell, RefCell};

use crate::allowed_url::Href;
use crate::aspect_ratio::AspectRatio;
use crate::bbox::BoundingBox;
use crate::drawing_ctx::{ClipMode, DrawingCtx};
use crate::error::{NodeError, RenderingError};
use crate::float_eq_cairo::ApproxEqCairo;
use crate::length::{LengthHorizontal, LengthVertical};
use crate::node::*;
use crate::parsers::{ParseError, ParseValue};
use crate::property_bag::PropertyBag;
use crate::rect::RectangleExt;
use crate::viewbox::ViewBox;

#[derive(Default)]
pub struct NodeImage {
    aspect: Cell<AspectRatio>,
    x: Cell<LengthHorizontal>,
    y: Cell<LengthVertical>,
    w: Cell<LengthHorizontal>,
    h: Cell<LengthVertical>,
    href: RefCell<Option<Href>>,
}

impl NodeTrait for NodeImage {
    fn set_atts(&self, _: Option<&RsvgNode>, pbag: &PropertyBag<'_>) -> NodeResult {
        for (attr, value) in pbag.iter() {
            match attr {
                local_name!("x") => self.x.set(attr.parse(value)?),
                local_name!("y") => self.y.set(attr.parse(value)?),
                local_name!("width") => self
                    .w
                    .set(attr.parse_and_validate(value, LengthHorizontal::check_nonnegative)?),
                local_name!("height") => self
                    .h
                    .set(attr.parse_and_validate(value, LengthVertical::check_nonnegative)?),

                local_name!("preserveAspectRatio") => self.aspect.set(attr.parse(value)?),

                // "path" is used by some older Adobe Illustrator versions
                local_name!("xlink:href") | local_name!("path") => {
                    let href = Href::parse(value).map_err(|_| {
                        NodeError::parse_error(attr, ParseError::new("could not parse href"))
                    })?;

                    *self.href.borrow_mut() = Some(href);
                }

                _ => (),
            }
        }

        Ok(())
    }

    fn overflow_hidden(&self) -> bool {
        true
    }

    fn draw(
        &self,
        node: &RsvgNode,
        cascaded: &CascadedValues<'_>,
        draw_ctx: &mut DrawingCtx,
        clipping: bool,
    ) -> Result<(), RenderingError> {
        let values = cascaded.get();
        let params = draw_ctx.get_view_params();

        let x = self.x.get().normalize(values, &params);
        let y = self.y.get().normalize(values, &params);
        let w = self.w.get().normalize(values, &params);
        let h = self.h.get().normalize(values, &params);

        if w.approx_eq_cairo(&0.0) || h.approx_eq_cairo(&0.0) {
            return Ok(());
        }

        draw_ctx.with_discrete_layer(node, values, clipping, &mut |dc| {
            let surface = if let Some(Href::PlainUrl(ref url)) = *self.href.borrow() {
                dc.lookup_image(&url)?
            } else {
                return Ok(());
            };

            let aspect = self.aspect.get();

            let clip_mode = if !values.is_overflow() && aspect.is_slice() {
                Some(ClipMode::ClipToViewport)
            } else {
                None
            };

            let image_width = surface.width();
            let image_height = surface.height();
            if clipping || image_width == 0 || image_height == 0 {
                return Ok(());
            }

            // The bounding box for <image> is decided by the values of x, y, w, h and not by
            // the final computed image bounds.
            let bbox = BoundingBox::new(&dc.get_cairo_context().get_matrix()).with_rect(Some(
                cairo::Rectangle {
                    x,
                    y,
                    width: w,
                    height: h,
                },
            ));

            dc.with_saved_cr(&mut |dc| {
                let cr = dc.get_cairo_context();

                let image_width = f64::from(image_width);
                let image_height = f64::from(image_height);

                if let Some(_params) = dc.push_new_viewport(
                    Some(ViewBox::new(0.0, 0.0, image_width, image_height)),
                    &Rectangle::new(x, y, w, h),
                    aspect,
                    clip_mode,
                ) {
                    // We need to set extend appropriately, so can't use cr.set_source_surface().
                    //
                    // If extend is left at its default value (None), then bilinear scaling uses
                    // transparency outside of the image producing incorrect results.
                    // For example, in svg1.1/filters-blend-01-b.svgthere's a completely
                    // opaque 100×1 image of a gradient scaled to 100×98 which ends up
                    // transparent almost everywhere without this fix (which it shouldn't).
                    let ptn = surface.to_cairo_pattern();
                    ptn.set_extend(cairo::Extend::Pad);
                    cr.set_source(&ptn);

                    // Clip is needed due to extend being set to pad.
                    cr.rectangle(0.0, 0.0, image_width, image_height);
                    cr.clip();

                    cr.paint();
                }

                Ok(())
            })
            .and_then(|()| {
                dc.insert_bbox(&bbox);
                Ok(())
            })
        })
    }
}
