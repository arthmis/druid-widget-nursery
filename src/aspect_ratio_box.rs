use druid::widget::prelude::*;
use druid::Data;

/// A widget that changes size dynamically; the dynamic analogue to [`SizedBox`].
///
/// If given a child, this widget forces the child to have a variable width and/or height.
///
/// If not given a child, The box will try to size itself as a fraction of the parent's
/// box constraints. If height or width is not set, it will be treated as zero.
///
/// [`SizedBox`]: druid::widget::SizedBox
pub struct AspectRatioBox<T> {
    inner: Option<Box<dyn Widget<T>>>,
    ratio: f64,
}

impl<T> AspectRatioBox<T> {
    /// Create container with child, and both width and height not set.
    pub fn new(inner: impl Widget<T> + 'static, ratio: f64) -> Self {
        Self {
            inner: Some(Box::new(inner)),
            ratio,
        }
    }

    /// Create container without child, and the ratio set to 1.0.
    fn empty() -> Self {
        Self {
            inner: None,
            ratio: 1.0,
        }
    }

    /// Builder-style method for setting the ratio.
    ///
    /// The ratio has to be a value between 0 and 1, excluding 0. It will be clamped
    /// to those values if they exceed the bounds. If the ratio is 0, then the ratio
    /// will become 1.
    fn with_ratio(mut self, mut ratio: f64) -> Self {
        ratio = f64::clamp(0.0, 1.0, ratio);
        if ratio == 0.0 {
            ratio = 1.0;
        }
        self.ratio = ratio;
        self
    }

    /// Set the ratio of the box.
    ///
    /// The ratio has to be a value between 0 and 1, excluding 0. It will be clamped
    /// to those values if they exceed the bounds. If the ratio is 0, then the ratio
    /// will become 1.
    pub fn set_ratio(&mut self, mut ratio: f64) {
        ratio = f64::clamp(0.0, 1.0, ratio);
        if ratio == 0.0 {
            ratio = 1.0;
        }
        self.ratio = ratio;
    }

    // /// Determine the constraints that will be used for inner widget.
    // fn inner_constraints(&self, bc: &BoxConstraints) -> BoxConstraints {
    //     // if we have a width/height, multiply it by bc.max to get new width/height
    //     // of widget and clamp on that value
    //     // if we don't have width/height, box constraints stay the same
    //     let (min_width, max_width) = match self.width_ratio {
    //         Some(width) => {
    //             let w = width * bc.max().width;
    //             (w, w)
    //         }
    //         None => (bc.min().width, bc.max().width),
    //     };

    //     let (min_height, max_height) = match self.height_ratio {
    //         Some(height) => {
    //             let h = height * bc.max().height;
    //             (h, h)
    //         }
    //         None => (bc.min().height, bc.max().height),
    //     };

    //     BoxConstraints::new(
    //         Size::new(min_width, min_height),
    //         Size::new(max_width, max_height),
    //     )
    // }
}

impl<T: Data> Widget<T> for AspectRatioBox<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Some(ref mut inner) = self.inner {
            inner.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let Some(ref mut inner) = self.inner {
            inner.lifecycle(ctx, event, data, env)
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if let Some(ref mut inner) = self.inner {
            inner.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("DynamicSizedBox");

        // let mut bc = bc.loosen();
        // dbg!(&bc);
        let (mut width, mut height) = (bc.max().width, bc.max().height);
        // this means we want the height to be the larger value
        // height and width are the max box constraints
        // if ratio is below 1 then the height of the box has the be the largest dimension
        // the width will then be a height * ratio
        let bc = if self.ratio < 1.0 {
            if (height >= width && height * self.ratio <= width) || width > height {
                width = height * self.ratio;
            } else if height >= width && height * self.ratio > width {
                height = width / self.ratio;
            }
            BoxConstraints::tight(Size::new(width, height))
        }
        // this means we want the width to be the larger value
        // if the ratio is above one then the width of the box has to be the largest dimension
        // the height will then be the width / ratio
        else if self.ratio > 1.0 {
            if width > height && height * self.ratio < width {
                width = height * self.ratio;
                // height = width / self.ratio;
            } else if (width > height && height * self.ratio > width) || height > width {
                height = width / self.ratio;
            }
            // dbg!(height, width);
            BoxConstraints::tight(Size::new(width, height))
        }
        // the aspect ratio is 1:1 which means we want a square
        // we take the minimum between the width and height and constrain to that min
        else {
            let min = width.min(height);
            BoxConstraints::tight(Size::new(min, min))
        };
        dbg!(&bc);

        // let inner_bc = self.inner_constraints(&bc);
        let size = match self.inner.as_mut() {
            Some(inner) => inner.layout(ctx, &bc, data, env),
            None => bc.max(),
        };
        // let size = bc.max();
        if let Some(ref mut inner) = self.inner {
            inner.layout(ctx, &bc, data, env);
        }

        if size.width.is_infinite() {
            log::warn!("DynamicSizedBox is returning an infinite width.");
        }

        if size.height.is_infinite() {
            log::warn!("DynamicSizedBox is returning an infinite height.");
        }

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if let Some(ref mut inner) = self.inner {
            inner.paint(ctx, data, env);
        }
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.as_ref().and_then(|inner| inner.id())
    }
}
