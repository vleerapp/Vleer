use std::{cell::RefCell, rc::Rc};

use gpui::*;

use crate::ui::variables::Variables;

type ClickHandler = dyn FnMut(f32, &mut Window, &mut App);

pub struct Slider {
    pub(self) id: Option<ElementId>,
    pub(self) style: StyleRefinement,
    pub(self) value: f32,
    pub(self) on_change: Option<Rc<RefCell<ClickHandler>>>,
    pub(self) hitbox: Option<Hitbox>,
}

impl Slider {
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn on_change(mut self, func: impl FnMut(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(RefCell::new(func)));
        self
    }
}

impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for Slider {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Slider {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        (window.request_layout(style, [], cx), ())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        _: &mut App,
    ) -> Self::PrepaintState {
        self.hitbox = Some(window.insert_hitbox(bounds, HitboxBehavior::Normal));
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let variables = cx.global::<Variables>();

        window.set_cursor_style(CursorStyle::PointingHand, self.hitbox.as_ref().unwrap());

        let thumb_width = px(10.0);
        let thumb_height = px(16.0);

        let thumb_x = bounds.origin.x + (bounds.size.width - thumb_width) * self.value;
        let thumb_y = bounds.origin.y + (bounds.size.height - thumb_height) / 2.0;

        let dash_size = px(4.0);
        let gap_size = px(4.0);
        let line_height = px(2.0);
        let line_y = bounds.origin.y + (bounds.size.height - line_height) / 2.0;

        let mut x = bounds.origin.x;
        let end_x = thumb_x + thumb_width / 2.0;

        while x < end_x {
            let dash_end = (x + dash_size).min(end_x);
            let dash_width = dash_end - x;

            if dash_width > px(0.0) {
                let dash_bounds = Bounds {
                    origin: Point { x, y: line_y },
                    size: Size {
                        width: dash_width,
                        height: line_height,
                    },
                };

                window.paint_quad(quad(
                    dash_bounds,
                    Corners::default(),
                    variables.accent,
                    Edges::all(px(0.0)),
                    rgb(0x000000),
                    BorderStyle::Solid,
                ));
            }

            x = x + dash_size + gap_size;
        }

        let thumb_bounds = Bounds {
            origin: Point {
                x: thumb_x,
                y: thumb_y,
            },
            size: Size {
                width: thumb_width,
                height: thumb_height,
            },
        };

        window.paint_quad(quad(
            thumb_bounds,
            Corners::default(),
            variables.text,
            Edges::all(px(0.0)),
            rgb(0x000000),
            BorderStyle::Solid,
        ));

        if let Some(func) = self.on_change.as_ref() {
            window.with_optional_element_state(
                id,
                move |v: Option<Option<Rc<RefCell<bool>>>>, cx| {
                    let mouse_in = v.flatten().unwrap_or_else(|| Rc::new(RefCell::new(false)));
                    let func = func.clone();
                    let func_copy = func.clone();

                    let mouse_in_1 = mouse_in.clone();

                    cx.on_mouse_event(move |ev: &MouseDownEvent, _, window, cx| {
                        if !bounds.contains(&ev.position) {
                            return;
                        }

                        window.prevent_default();
                        cx.stop_propagation();

                        let relative = ev.position - bounds.origin;
                        let relative_x: f32 = relative.x.into();
                        let width: f32 = bounds.size.width.into();
                        let value = (relative_x / width).clamp(0.0, 1.0);

                        (func.borrow_mut())(value, window, cx);
                        (*mouse_in_1.borrow_mut()) = true;
                    });

                    let mouse_in_2 = mouse_in.clone();

                    cx.on_mouse_event(move |ev: &MouseMoveEvent, _, window, cx| {
                        if *mouse_in_2.borrow() {
                            let relative = ev.position - bounds.origin;
                            let relative_x: f32 = relative.x.into();
                            let width: f32 = bounds.size.width.into();
                            let value = (relative_x / width).clamp(0.0, 1.0);

                            (func_copy.borrow_mut())(value, window, cx);
                        }
                    });

                    let mouse_in_3 = mouse_in.clone();

                    cx.on_mouse_event(move |_: &MouseUpEvent, _, _, _| {
                        (*mouse_in_3.borrow_mut()) = false;
                    });

                    ((), Some(mouse_in))
                },
            )
        }
    }
}

pub fn slider() -> Slider {
    Slider {
        id: None,
        style: StyleRefinement::default(),
        value: 0.0,
        on_change: None,
        hitbox: None,
    }
}
