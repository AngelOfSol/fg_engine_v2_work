use imgui::*;
use std::convert::{TryFrom, TryInto};
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
    fn rearrangable_list_box<T, F: Fn(&T) -> ImString>(
        &self,
        label: &ImStr,
        idx: &mut Option<usize>,
        items: &mut Vec<T>,
        display: F,
        height_in_items: i32,
    ) -> bool;

    fn input_text_left(&self, label: &ImStr, buf: &mut ImString);
    fn list_box_owned<'p>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [ImString],
        height_in_items: i32,
    ) -> bool;

    fn input_whole<I: Copy + TryInto<i32> + TryFrom<i32>>(
        &self,
        label: &ImStr,
        value: &mut I,
    ) -> Result<bool, String>;
    fn input_string(&self, label: &ImStr, value: &mut String) -> bool;
}

impl<'a> UiExtensions for Ui<'a> {
    fn rearrangable_list_box<T, F: Fn(&T) -> ImString>(
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
            &items.iter().map(display).collect::<Vec<_>>().iter().collect::<Vec<_>>(),
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
    fn input_text_left(&self, label: &ImStr, buf: &mut ImString) {
        unsafe { imgui_sys::igAlignTextToFramePadding() };
        self.text(label);
        self.same_line(0.0);
        self.push_id(label);
        self.input_text(im_str!("###Input"), buf).build();
        self.pop_id();
    }
    fn input_string(&self, label: &ImStr, value: &mut String) -> bool {
        let mut buffer = im_str_owned!("{}", value.clone());
        buffer.reserve_exact(16);
        let changed = self.input_text(label, &mut buffer).build();
        if changed {
            *value = buffer.to_str().to_owned();
        }
        changed
    }

    fn list_box_owned<'p>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [ImString],
        height_in_items: i32,
    ) -> bool {
        self.list_box(
            label,
            current_item,
            &items.iter().collect::<Vec<_>>(),
            height_in_items,
        )
    }
}