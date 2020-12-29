use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq)]
pub enum TimelineInsertError {
    FrameAlreadyExists,
    OutOfBounds,
    NoStartFrame,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, Deserialize)]
pub struct Timeline<T> {
    data: Vec<(usize, T)>,
    duration: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Surrounding<T> {
    None,
    End { start: T, duration: usize },
    Pair { start: T, end: T },
}

impl<T> Surrounding<(usize, T)> {
    pub fn is_between(&self, frame: usize) -> bool {
        match self {
            Surrounding::None => false,
            Surrounding::End { duration, start } => frame >= start.0 && frame < start.0 + duration,
            Surrounding::Pair { start, end } => frame >= start.0 && frame < end.0,
        }
    }
}

impl<T: Default> Default for Timeline<T> {
    fn default() -> Self {
        Self {
            data: vec![(0, T::default())],
            duration: 1,
        }
    }
}

impl<T: Default> Timeline<T> {
    pub fn new(duration: usize) -> Self {
        Self {
            duration,
            ..Default::default()
        }
    }
}

impl<T> Timeline<T> {
    pub fn with_data(data: Vec<(usize, T)>, duration: usize) -> Result<Self, TimelineInsertError> {
        let mut ret = Self {
            data: vec![],
            duration,
        };
        for (frame, data) in data.into_iter() {
            ret.insert_frame(frame, data)?;
        }

        if ret.data[0].0 != 0 {
            Err(TimelineInsertError::NoStartFrame)
        } else {
            Ok(ret)
        }
    }

    pub fn get(&self, target_frame: usize) -> (usize, &T) {
        self.data
            .iter()
            .rev()
            .find(|(frame, _)| target_frame >= *frame)
            .map(|(frame, data)| (*frame, data))
            .unwrap()
    }
    pub fn get_mut(&mut self, target_frame: usize) -> (usize, &mut T) {
        self.data
            .iter_mut()
            .rev()
            .find(|(frame, _)| target_frame >= *frame)
            .map(|(frame, data)| (*frame, data))
            .unwrap()
    }
    pub fn duration(&self) -> usize {
        self.duration
    }
    /// Removes every frame outside of the new duration.
    pub fn set_duration(&mut self, duration: usize) {
        if duration == 0 {
            panic!("Can't have a 0 duration timeline.");
        }
        self.duration = duration;
    }

    #[allow(dead_code)]
    pub fn clean(&mut self) {
        let duration = self.duration;
        self.data.retain(|item| item.0 < duration);
    }

    pub fn remove_frame(&mut self, target_frame: usize) -> Option<T> {
        if target_frame == 0 {
            return None;
        }
        let idx = self
            .data
            .iter()
            .position(|(frame, _)| target_frame == *frame);

        idx.map(|idx| self.data.remove(idx).1)
    }

    pub fn insert_frame(
        &mut self,
        target_frame: usize,
        data: T,
    ) -> Result<(), TimelineInsertError> {
        if target_frame >= self.duration {
            Err(TimelineInsertError::OutOfBounds)
        } else if self.data.iter().any(|item| item.0 == target_frame) {
            Err(TimelineInsertError::FrameAlreadyExists)
        } else {
            let index = self
                .data
                .iter()
                .rev()
                .map(|item| item.0)
                .position(|frame| target_frame >= frame)
                .map(|idx| self.data.len() - idx)
                .unwrap_or(0);
            self.data.insert(index, (target_frame, data));
            Ok(())
        }
    }
    pub fn insert_force(&mut self, target_frame: usize, data: T) {
        if self.duration < target_frame {
            self.duration = target_frame + 1;
        }
        if self.data.iter().any(|item| item.0 == target_frame) {
            let index = self
                .data
                .iter()
                .position(|item| item.0 == target_frame)
                .unwrap();
            self.data[index].1 = data;
        } else {
            let index = self
                .data
                .iter()
                .rev()
                .map(|item| item.0)
                .position(|frame| target_frame >= frame)
                .map(|idx| self.data.len() - idx)
                .unwrap_or(0);
            self.data.insert(index, (target_frame, data));
        }
    }

    /// Returns the two frames surrounding the target frame.  The last frame is returned by itself along with
    /// the remaining duration.  If the target_frame is outside of the timeline, it returns Surronding::None.
    pub fn surrounding(&self, target_frame: usize) -> Surrounding<(usize, &T)> {
        if target_frame >= self.duration {
            return Surrounding::None;
        }
        if let Some((index, _)) = self
            .data
            .iter()
            .enumerate()
            .rev()
            .find(|(_, (frame, _))| target_frame >= *frame)
        {
            if index == self.data.len() - 1 {
                Surrounding::End {
                    duration: self.duration - self.data[index].0,
                    start: self
                        .data
                        .get(index)
                        .map(|(idx, item)| (*idx, item))
                        .unwrap(),
                }
            } else {
                Surrounding::Pair {
                    start: self
                        .data
                        .get(index)
                        .map(|(idx, item)| (*idx, item))
                        .unwrap(),
                    end: self
                        .data
                        .get(index + 1)
                        .map(|(idx, item)| (*idx, item))
                        .unwrap(),
                }
            }
        } else {
            Surrounding::None
        }
    }

    /// Returns true if `frame` is a key frame.
    pub fn has_keyframe(&self, frame: usize) -> bool {
        self.data.iter().any(|item| item.0 == frame)
    }

    pub fn frames(&self) -> impl Iterator<Item = &T> {
        self.data.iter().map(|(_, data)| data)
    }
    pub fn frames_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, data)| data)
    }
}

pub mod inspect {
    use super::*;
    use imgui::*;
    use inspect_design::traits::{Inspect, InspectMut};

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TimelineState<T> {
        //
        hovered_frame: Option<usize>,
        selected_frame: usize,
        selected_state: T,
        start_drag: Option<usize>,
        duration: <usize as Inspect>::State,
    }
    struct UiRect {
        top_left: [f32; 2],
        bottom_right: [f32; 2],
    }
    struct SelectionFlags {
        is_hovered: bool,
        in_selected_range: bool,
        in_hovered_range: bool,
        is_selected: bool,
        is_keyframe: Option<usize>,
    }
    fn draw(rect: UiRect, flags: SelectionFlags, draw_list: &WindowDrawList, ui: &Ui<'_>) {
        draw_list
            .add_rect(
                rect.top_left,
                rect.bottom_right,
                ui.style_color(if flags.is_selected || flags.is_hovered {
                    StyleColor::FrameBgActive
                } else if flags.in_selected_range || flags.in_hovered_range {
                    StyleColor::FrameBgHovered
                } else {
                    StyleColor::FrameBg
                }),
            )
            .thickness(1.0)
            .filled(true)
            .build();
        if let Some(frame) = flags.is_keyframe {
            const OFFSET: f32 = 3.0;
            let text_size = ui.calc_text_size(&im_str!("{}", frame), false, 100.0);

            let top_left = [
                rect.top_left[0] + OFFSET,
                (rect.top_left[1] + rect.bottom_right[1] - text_size[1]) / 2.0,
            ];
            if text_size[0] + OFFSET < ui.item_rect_size()[0] {
                draw_list.add_text(top_left, [1.0, 1.0, 1.0, 1.0], &frame.to_string());
            }
        }
        let inlay: f32 = flags.is_keyframe.map(|_| 3.0).unwrap_or(6.0);
        let top_left = [rect.top_left[0].round(), rect.top_left[1] + inlay];
        let bottom_right = [rect.top_left[0].round(), rect.bottom_right[1] - inlay];
        draw_list
            .add_line(
                top_left,
                bottom_right,
                [1.0, 1.0, 1.0, flags.is_keyframe.map(|_| 0.8).unwrap_or(0.2)],
            )
            .thickness(1.0)
            .build();
    }

    fn draw_frame_info<T>(selected: usize, surrounding: Surrounding<(usize, T)>, ui: &Ui<'_>) {
        match surrounding {
            Surrounding::None => {}
            Surrounding::End { start, duration } => ui.text(&im_str!(
                "start: f{}, selected: f{}, end: f{}, length: {}f",
                start.0,
                selected,
                start.0 + duration - 1,
                duration
            )),
            Surrounding::Pair { start, end } => ui.text(&im_str!(
                "start: f{}, selected: f{}, end: f{}, length: {}f",
                start.0,
                selected,
                end.0 - 1,
                end.0 - start.0
            )),
        }
    }

    impl<T> Inspect for Timeline<T>
    where
        T: Inspect,
    {
        type State = TimelineState<T::State>;
        const FLATTEN: bool = false;
        fn inspect(&self, label: &str, state: &mut Self::State, ui: &Ui<'_>) {
            let id = ui.push_id(label);
            let item_width = ui.push_item_width(-1.0);
            let total_width = ui.calc_item_width();
            item_width.pop(ui);
            let frame_width = total_width / (self.duration as f32);
            let total_height = 22.0;

            ui.group(|| {
                let draw_list = ui.get_window_draw_list();
                let remove_item_spacing = ui.push_style_var(StyleVar::ItemSpacing([0.0, 0.0]));
                let mut hovered = None;
                for frame in 0..self.duration {
                    ui.invisible_button(&im_str!("{}", frame), [frame_width, total_height]);
                    ui.same_line(0.0);

                    let is_hovered =
                        ui.is_mouse_hovering_rect(ui.item_rect_min(), ui.item_rect_max());

                    if ui.is_item_clicked(MouseButton::Left) {
                        state.selected_frame = frame;
                    }

                    if is_hovered {
                        hovered = Some(frame);
                    }

                    let surrounding = self.surrounding(frame);
                    draw(
                        UiRect {
                            top_left: ui.item_rect_min(),
                            bottom_right: ui.item_rect_max(),
                        },
                        SelectionFlags {
                            is_hovered,
                            in_selected_range: surrounding.is_between(state.selected_frame),
                            in_hovered_range: state
                                .hovered_frame
                                .map(|frame| surrounding.is_between(frame))
                                .unwrap_or(false),
                            is_selected: state.selected_frame == frame,
                            is_keyframe: if self.has_keyframe(frame) {
                                Some(frame)
                            } else {
                                None
                            },
                        },
                        &draw_list,
                        ui,
                    );
                }

                state.hovered_frame = hovered;
                if ui.is_mouse_dragging(MouseButton::Left) {
                    if let Some(hovered) = hovered {
                        state.selected_frame = hovered;
                    }
                }
                ui.new_line();
                remove_item_spacing.pop(ui);
            });

            self.duration.inspect("duration", &mut state.duration, ui);
            ui.separator();

            draw_frame_info(
                state.selected_frame,
                self.surrounding(state.selected_frame),
                ui,
            );

            let (frame, item) = self.get(state.selected_frame);
            if T::FLATTEN {
                item.inspect("selected", &mut state.selected_state, ui);
            } else {
                ui.text(&im_str!("selected: {"));
                ui.indent();
                ChildWindow::new(&im_str!("selected")).build(ui, || {
                    item.inspect(&format!("frame {}", frame), &mut state.selected_state, ui);
                });
                ui.unindent();
                ui.text(&im_str!("}"));
            }

            id.pop(ui);
        }
    }
    impl<T> InspectMut for Timeline<T>
    where
        T: Clone + InspectMut,
    {
        fn inspect_mut(&mut self, label: &str, state: &mut Self::State, ui: &Ui<'_>) {
            let id = ui.push_id(label);
            let item_width = ui.push_item_width(-1.0);
            let total_width = ui.calc_item_width();
            item_width.pop(ui);
            let frame_width = total_width / (self.duration as f32);
            let total_height = 22.0;

            ui.group(|| {
                let draw_list = ui.get_window_draw_list();
                let remove_item_spacing = ui.push_style_var(StyleVar::ItemSpacing([0.0, 0.0]));
                let mut hovered = None;
                let mut to_add = None;
                let mut to_remove = None;
                for frame in 0..self.duration {
                    ui.invisible_button(&im_str!("{}", frame), [frame_width, total_height]);
                    let is_hovered =
                        ui.is_mouse_hovering_rect(ui.item_rect_min(), ui.item_rect_max());
                    if ui.is_item_clicked(MouseButton::Left) {
                        state.selected_frame = frame;
                        if ui.is_mouse_double_clicked(MouseButton::Left) {
                            to_add = Some(frame);
                        }
                        if self.has_keyframe(frame) && frame != 0 {
                            state.start_drag = Some(frame);
                        }
                    }
                    if ui.is_item_clicked(MouseButton::Right) {
                        to_remove = Some(frame);
                    }
                    if is_hovered {
                        hovered = Some(frame);
                    }
                    ui.same_line(0.0);

                    let surrounding = self.surrounding(frame);
                    draw(
                        UiRect {
                            top_left: ui.item_rect_min(),
                            bottom_right: ui.item_rect_max(),
                        },
                        SelectionFlags {
                            is_hovered,
                            in_selected_range: surrounding.is_between(state.selected_frame),
                            in_hovered_range: state
                                .hovered_frame
                                .map(|frame| surrounding.is_between(frame))
                                .unwrap_or(false),
                            is_selected: state.selected_frame == frame,
                            is_keyframe: if self.has_keyframe(frame) {
                                Some(frame)
                            } else {
                                None
                            },
                        },
                        &draw_list,
                        ui,
                    );
                }

                state.hovered_frame = hovered;
                if let Some(target_frame) = to_add {
                    let _ = self.insert_frame(
                        target_frame,
                        match self.surrounding(target_frame) {
                            Surrounding::None => unreachable!(),
                            Surrounding::End { start, .. } | Surrounding::Pair { start, .. } => {
                                start.1.clone()
                            }
                        },
                    );
                }
                if let Some(target_frame) = to_remove {
                    self.remove_frame(target_frame);
                }
                if ui.is_mouse_dragging(MouseButton::Left) {
                    if let Some(hovered) = hovered {
                        state.selected_frame = hovered;
                    }
                    if let Some(frame) = state.start_drag {
                        if !self.has_keyframe(state.selected_frame) {
                            let data = self.remove_frame(frame).unwrap();
                            self.insert_force(state.selected_frame, data);
                            state.start_drag = Some(state.selected_frame);
                        }
                    }
                }
                ui.new_line();
                remove_item_spacing.pop(ui);
            });
            if !ui.is_item_hovered() {
                state.start_drag = None;
            }

            let mut duration = self.duration;
            duration.inspect_mut("duration", &mut (), ui);
            if duration > 0 {
                self.set_duration(duration);
            }
            ui.separator();

            draw_frame_info(
                state.selected_frame,
                self.surrounding(state.selected_frame),
                ui,
            );

            let (frame, item) = self.get_mut(state.selected_frame);
            if T::FLATTEN {
                item.inspect_mut("selected", &mut state.selected_state, ui);
            } else {
                ui.text(&im_str!("selected: {"));
                ui.indent();
                item.inspect_mut(&format!("frame {}", frame), &mut state.selected_state, ui);
                ui.unindent();
                ui.text(&im_str!("}"));
            }

            id.pop(ui);
        }
    }

    pub fn inspect_mut_custom<T: Clone, S, F: FnOnce(usize, &mut T)>(
        data: &mut Timeline<T>,
        label: &str,
        state: &mut TimelineState<S>,
        ui: &Ui<'_>,
        inspect: F,
    ) {
        let id = ui.push_id(label);
        let item_width = ui.push_item_width(-1.0);
        let total_width = ui.calc_item_width();
        item_width.pop(ui);
        let frame_width = total_width / (data.duration as f32);
        let total_height = 22.0;

        ui.group(|| {
            let draw_list = ui.get_window_draw_list();
            let remove_item_spacing = ui.push_style_var(StyleVar::ItemSpacing([0.0, 0.0]));
            let mut hovered = None;
            let mut to_add = None;
            let mut to_remove = None;
            for frame in 0..data.duration {
                ui.invisible_button(&im_str!("{}", frame), [frame_width, total_height]);
                let is_hovered = ui.is_mouse_hovering_rect(ui.item_rect_min(), ui.item_rect_max());
                if ui.is_item_clicked(MouseButton::Left) {
                    state.selected_frame = frame;
                    if ui.is_mouse_double_clicked(MouseButton::Left) {
                        to_add = Some(frame);
                    }
                    if data.has_keyframe(frame) && frame != 0 {
                        state.start_drag = Some(frame);
                    }
                }
                if ui.is_item_clicked(MouseButton::Right) {
                    to_remove = Some(frame);
                }
                if is_hovered {
                    hovered = Some(frame);
                }
                ui.same_line(0.0);

                let surrounding = data.surrounding(frame);
                draw(
                    UiRect {
                        top_left: ui.item_rect_min(),
                        bottom_right: ui.item_rect_max(),
                    },
                    SelectionFlags {
                        is_hovered,
                        in_selected_range: surrounding.is_between(state.selected_frame),
                        in_hovered_range: state
                            .hovered_frame
                            .map(|frame| surrounding.is_between(frame))
                            .unwrap_or(false),
                        is_selected: state.selected_frame == frame,
                        is_keyframe: if data.has_keyframe(frame) {
                            Some(frame)
                        } else {
                            None
                        },
                    },
                    &draw_list,
                    ui,
                );
            }

            state.hovered_frame = hovered;
            if let Some(target_frame) = to_add {
                let _ = data.insert_frame(
                    target_frame,
                    match data.surrounding(target_frame) {
                        Surrounding::None => unreachable!(),
                        Surrounding::End { start, .. } | Surrounding::Pair { start, .. } => {
                            start.1.clone()
                        }
                    },
                );
            }
            if let Some(target_frame) = to_remove {
                data.remove_frame(target_frame);
            }
            if ui.is_mouse_dragging(MouseButton::Left) {
                if let Some(hovered) = hovered {
                    state.selected_frame = hovered;
                }
                if let Some(frame) = state.start_drag {
                    if !data.has_keyframe(state.selected_frame) {
                        let value = data.remove_frame(frame).unwrap();
                        data.insert_force(state.selected_frame, value);
                        state.start_drag = Some(state.selected_frame);
                    }
                }
            }
            ui.new_line();
            remove_item_spacing.pop(ui);
        });
        if !ui.is_item_hovered() {
            state.start_drag = None;
        }

        let mut duration = data.duration;
        duration.inspect_mut("duration", &mut (), ui);
        if duration > 0 {
            data.set_duration(duration);
        }
        ui.separator();

        draw_frame_info(
            state.selected_frame,
            data.surrounding(state.selected_frame),
            ui,
        );

        let (frame, item) = data.get_mut(state.selected_frame);
        ChildWindow::new(&im_str!("selected")).build(ui, || {
            inspect(frame, item);
        });

        id.pop(ui);
    }
}

impl<T> std::ops::Index<usize> for Timeline<T> {
    type Output = T;
    fn index(&self, frame: usize) -> &Self::Output {
        self.get(frame).1
    }
}

#[cfg(test)]
mod test {
    use super::{Surrounding, Timeline, TimelineInsertError};
    #[test]
    fn with_data_valid() {
        assert_eq!(
            Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5),
            Ok(Timeline {
                data: vec![(0, 0), (2, 2), (4, 4)],
                duration: 5
            })
        );
    }
    #[test]
    fn with_data_valid_out_of_order() {
        assert_eq!(
            Timeline::with_data(vec![(0, 0), (4, 4), (2, 2)], 5),
            Ok(Timeline {
                data: vec![(0, 0), (2, 2), (4, 4)],
                duration: 5
            })
        );
    }
    #[test]
    fn with_data_invalid_duration() {
        assert_eq!(
            Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 3),
            Err(TimelineInsertError::OutOfBounds)
        );
    }
    #[test]
    fn with_data_invalid_frame() {
        assert_eq!(
            Timeline::with_data(vec![(0, 0), (2, 2), (2, 4)], 3),
            Err(TimelineInsertError::FrameAlreadyExists)
        );
    }

    #[test]
    fn get_surrounding() {
        let data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 7).unwrap();
        assert_eq!(
            data.surrounding(0),
            Surrounding::Pair {
                start: (0, &0),
                end: (2, &2),
            },
        );
        assert_eq!(data.surrounding(0), data.surrounding(1),);
        assert_eq!(data.surrounding(3), data.surrounding(2),);
        assert_eq!(
            data.surrounding(3),
            Surrounding::Pair {
                start: (2, &2),
                end: (4, &4),
            },
        );
        assert_eq!(
            data.surrounding(4),
            Surrounding::End {
                start: (4, &4),
                duration: 3
            },
        );
        assert_eq!(
            data.surrounding(5),
            Surrounding::End {
                start: (4, &4),
                duration: 3
            },
        );
        assert_eq!(data.surrounding(30), Surrounding::None,);
        //
    }

    #[test]
    fn has_keyframe() {
        let data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();
        assert!(data.has_keyframe(0));
        assert!(data.has_keyframe(2));
        assert!(data.has_keyframe(4));

        assert!(!data.has_keyframe(1));
        assert!(!data.has_keyframe(3));
    }

    #[test]
    fn insert_test() {
        let mut data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();
        assert_eq!(data.insert_frame(3, 3), Ok(()));
        assert_eq!(
            data,
            Timeline {
                data: vec![(0, 0), (2, 2), (3, 3), (4, 4)],
                duration: 5
            }
        );
    }
    #[test]
    fn insert_out_of_bounds_test() {
        let mut data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();
        assert_eq!(
            data.insert_frame(6, 3),
            Err(TimelineInsertError::OutOfBounds)
        );
    }
    #[test]
    fn insert_already_exists_test() {
        let mut data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();
        assert_eq!(
            data.insert_frame(2, 3),
            Err(TimelineInsertError::FrameAlreadyExists)
        );
    }
    #[test]
    fn force_insert_test() {
        let mut data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();
        data.insert_force(3, 3);
        assert_eq!(
            data,
            Timeline {
                data: vec![(0, 0), (2, 2), (3, 3), (4, 4)],
                duration: 5
            }
        );
    }
    #[test]
    fn force_insert_overwrite_test() {
        let mut data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();
        data.insert_force(2, 4);
        assert_eq!(
            data,
            Timeline {
                data: vec![(0, 0), (2, 4), (4, 4)],
                duration: 5
            }
        );
    }
    #[test]
    fn force_insert_expand_duration_test() {
        let mut data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();
        data.insert_force(6, 4);
        assert_eq!(
            data,
            Timeline {
                data: vec![(0, 0), (2, 2), (4, 4), (6, 4)],
                duration: 7
            }
        );
    }

    #[test]
    fn index_test() {
        let data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();

        for (idx, test) in (0..7).zip(vec![0, 0, 2, 2, 4, 4, 4].into_iter()) {
            assert_eq!(*data.get(idx).1, test);
        }
    }
    #[test]
    fn index_mut_test() {
        let mut data = Timeline::with_data(vec![(0, 0), (2, 2), (4, 4)], 5).unwrap();

        for (idx, test) in (0..7).zip(vec![0, 0, 2, 2, 4, 4, 4].into_iter()) {
            assert_eq!(*data.get_mut(idx).1, test);
        }
    }
}
