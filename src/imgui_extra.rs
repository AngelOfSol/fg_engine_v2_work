use crate::typedefs::collision;
use crate::typedefs::graphics;
use imgui::*;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::hash::Hash;
use strum::IntoEnumIterator;

#[macro_export]
macro_rules! im_str_owned {
    ($e:tt, $($arg:tt)*) => ({
        unsafe {
          imgui::ImString::from_utf8_with_nul_unchecked(
            format!(concat!($e, "\0"), $($arg)*).into_bytes())
        }
    });
}

pub trait UiExtensions {
    fn checkbox_set<T: Clone + Hash + PartialEq + Eq + Display>(
        &self,
        range: &[T],
        data: &mut HashSet<T>,
    );
    fn checkbox_hash<T: Clone + Hash + PartialEq + Eq>(
        &self,
        label: &ImStr,
        item: &T,
        data: &mut HashSet<T>,
    );

    fn input_vec2_float(&self, label: &ImStr, data: &mut graphics::Vec2) -> bool;
    fn input_vec2_whole(&self, label: &ImStr, data: &mut collision::Vec2);
    fn input_vec2_pixels(&self, label: &ImStr, data: &mut collision::Vec2);

    fn rearrangable_list_box<T, F: FnMut(&T) -> ImString>(
        &self,
        label: &ImStr,
        idx: &mut Option<usize>,
        items: &mut Vec<T>,
        display: F,
        height_in_items: i32,
    ) -> bool;
    #[allow(clippy::too_many_arguments)]
    fn new_delete_list_box<
        'items,
        T,
        Display: FnMut(&T) -> ImString,
        New: FnOnce() -> T,
        Delete: FnOnce(T),
    >(
        &self,
        label: &ImStr,
        idx: &mut Option<usize>,
        items: &'items mut Vec<T>,
        display: Display,
        new: New,
        delete: Delete,
        height_in_items: i32,
    ) -> (bool, Option<&'items mut T>);

    fn input_whole<I: Copy + TryInto<i32> + TryFrom<i32>>(
        &self,
        label: &ImStr,
        value: &mut I,
    ) -> Result<bool, String>;

    fn slider_whole<I: Copy + TryInto<i32> + TryFrom<i32>>(
        &self,
        label: &ImStr,
        value: &mut I,
        min: I,
        max: I,
    ) -> Result<bool, String>;
    fn input_string(&self, label: &ImStr, value: &mut String) -> bool;

    fn combo_items<T: PartialEq + Clone, L>(
        &self,
        label: &ImStr,
        value: &mut T,
        items: &[T],
        f: &L,
    ) -> bool
    where
        for<'b> L: Fn(&'b T) -> std::borrow::Cow<'b, ImStr>;
    fn combo_items_display<T: PartialEq + Clone + Display>(
        &self,
        label: &ImStr,
        value: &mut T,
        items: &[T],
    ) -> bool;

    fn combo_enum<T: PartialEq + Clone + IntoEnumIterator + Display>(
        &self,
        label: &ImStr,
        value: &mut T,
    ) -> bool;
}

impl<'a> UiExtensions for Ui<'a> {
    fn combo_items<T: PartialEq + Clone, L>(
        &self,
        label: &ImStr,
        value: &mut T,
        items: &[T],
        f: &L,
    ) -> bool
    where
        for<'b> L: Fn(&'b T) -> std::borrow::Cow<'b, ImStr>,
    {
        let mut idx = items.iter().position(|item| *item == *value).unwrap_or(0);

        if imgui::ComboBox::new(label).build_simple(self, &mut idx, &items, f) {
            *value = items[idx].clone();
            true
        } else {
            false
        }
    }
    fn combo_enum<T: PartialEq + Clone + IntoEnumIterator + Display>(
        &self,
        label: &ImStr,
        value: &mut T,
    ) -> bool {
        self.combo_items(label, value, &T::iter().collect::<Vec<_>>(), &|i| {
            im_str!("{}", i).into()
        })
    }
    fn combo_items_display<T: PartialEq + Clone + Display>(
        &self,
        label: &ImStr,
        value: &mut T,
        items: &[T],
    ) -> bool {
        self.combo_items(label, value, items, &|i| im_str!("{}", i).into())
    }

    fn checkbox_set<T: Clone + Hash + PartialEq + Eq + Display>(
        &self,
        range: &[T],
        data: &mut HashSet<T>,
    ) {
        for item in range {
            let mut buffer = data.contains(&item);
            if self.checkbox(&im_str!("{}", item), &mut buffer) {
                if buffer {
                    data.insert(item.clone());
                } else {
                    data.remove(item);
                }
            }
        }
    }
    fn checkbox_hash<T: Clone + Hash + PartialEq + Eq>(
        &self,
        label: &ImStr,
        item: &T,
        data: &mut HashSet<T>,
    ) {
        let mut buffer = data.contains(&item);
        if self.checkbox(label, &mut buffer) {
            if buffer {
                data.insert(item.clone());
            } else {
                data.remove(item);
            }
        }
    }
    fn rearrangable_list_box<T, F: FnMut(&T) -> ImString>(
        &self,
        label: &ImStr,
        idx: &mut Option<usize>,
        items: &mut Vec<T>,
        display: F,
        height_in_items: i32,
    ) -> bool {
        let mut buffer = idx.and_then(|item| i32::try_from(item).ok()).unwrap_or(-1);
        let ret = self.list_box(
            label,
            &mut buffer,
            &items
                .iter()
                .map(display)
                .collect::<Vec<_>>()
                .iter()
                .collect::<Vec<_>>(),
            height_in_items,
        );

        *idx = usize::try_from(buffer).ok();
        if let (Some(ref mut inside_idx), true) = (idx, !items.is_empty()) {
            let (up, down) = if *inside_idx == 0 {
                let temp = self.arrow_button(im_str!("Swap Down"), imgui::Direction::Down);
                (false, temp)
            } else if *inside_idx == items.len() - 1 {
                let temp = self.arrow_button(im_str!("Swap Up"), imgui::Direction::Up);
                (temp, false)
            } else {
                let up = self.arrow_button(im_str!("Swap Up"), imgui::Direction::Up);
                self.same_line(0.0);
                let down = self.arrow_button(im_str!("Swap Down"), imgui::Direction::Down);
                (up, down)
            };
            if up && *inside_idx != 0 {
                items.swap(*inside_idx, *inside_idx - 1);
                *inside_idx -= 1;
            } else if down && *inside_idx != items.len() - 1 {
                items.swap(*inside_idx, *inside_idx + 1);
                *inside_idx += 1;
            }
        };

        ret
    }

    fn input_whole<I: Copy + TryInto<i32> + TryFrom<i32>>(
        &self,
        label: &ImStr,
        value: &mut I,
    ) -> Result<bool, String> {
        let mut buffer = (*value)
            .try_into()
            .map_err(|_| "something happened".to_owned())?;
        let changed = self.input_int(label, &mut buffer).build();
        if changed {
            *value = I::try_from(buffer).map_err(|_| "something happened".to_owned())?;
        }
        Ok(changed)
    }

    fn slider_whole<I: Copy + TryInto<i32> + TryFrom<i32>>(
        &self,
        label: &ImStr,
        value: &mut I,
        min: I,
        max: I,
    ) -> Result<bool, String> {
        let min = min
            .try_into()
            .map_err(|_| "something happened".to_owned())?;
        let max = max
            .try_into()
            .map_err(|_| "something happened".to_owned())?;
        let mut buffer = (*value)
            .try_into()
            .map_err(|_| "something happened".to_owned())?;
        let changed = imgui::Slider::new(label)
            .range(min..=max)
            .build(self, &mut buffer);
        //self.slider_int(label, &mut buffer, min, max).build();
        if changed {
            *value = I::try_from(buffer).map_err(|_| "something happened".to_owned())?;
        }
        Ok(changed)
    }

    fn input_string(&self, label: &ImStr, value: &mut String) -> bool {
        let mut buffer = im_str_owned!("{}", value.clone());
        let changed = self
            .input_text(label, &mut buffer)
            .resize_buffer(true)
            .build();
        if changed {
            *value = buffer.to_str().to_owned();
        }
        changed
    }
    fn new_delete_list_box<
        'items,
        T,
        Display: FnMut(&T) -> ImString,
        New: FnOnce() -> T,
        Delete: FnOnce(T),
    >(
        &self,
        label: &ImStr,
        idx: &mut Option<usize>,
        items: &'items mut Vec<T>,
        display: Display,
        new: New,
        delete: Delete,
        height_in_items: i32,
    ) -> (bool, Option<&'items mut T>) {
        let mut ret = self.rearrangable_list_box(label, idx, items, display, height_in_items);
        if self.small_button(im_str!("New")) {
            ret = true;
            items.push(new());
            *idx = Some(items.len() - 1);
        }
        self.same_line(0.0);
        if self.small_button(im_str!("Delete")) {
            if let Some(idx) = idx {
                let item = items.remove(*idx);
                delete(item);
            }
            *idx = None;
        }

        (ret, idx.map(move |idx| &mut items[idx]))
    }

    fn input_vec2_float(&self, label: &ImStr, data: &mut graphics::Vec2) -> bool {
        let id = self.push_id(label);
        self.text(label);
        let ret = self.input_float(im_str!("X"), &mut data.x).build()
            || self.input_float(im_str!("Y"), &mut data.y).build();
        id.pop(self);
        ret
    }

    fn input_vec2_whole(&self, label: &ImStr, data: &mut collision::Vec2) {
        let id = self.push_id(label);
        self.text(label);
        let _ = self.input_whole(im_str!("X"), &mut data.x);
        let _ = self.input_whole(im_str!("Y"), &mut data.y);
        id.pop(self);
    }
    fn input_vec2_pixels(&self, label: &ImStr, data: &mut collision::Vec2) {
        *data /= 1_00;
        self.input_vec2_whole(label, data);
        *data *= 1_00;
    }
}
