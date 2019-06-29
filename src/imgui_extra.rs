use imgui::*;
use std::convert::{TryFrom, TryInto};
#[macro_export]
macro_rules! im_str_owned {
    ($e:tt, $($arg:tt)*) => ({
        unsafe {
          ImString::from_utf8_with_nul_unchecked(
            format!(concat!($e, "\0"), $($arg)*).into_bytes())
        }
    });
}


pub trait UiExtensions {
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
}

impl<'a> UiExtensions for Ui<'a> {
    fn input_whole<I: Copy + TryInto<i32> + TryFrom<i32>>(
        &self,
        label: &ImStr,
        value: &mut I,
    ) -> Result<bool, String> {
        let mut buffer = (*value)
            .try_into()
            .map_err(|_| "something happened".to_owned())?;
        let ret = self.input_int(label, &mut buffer).build();
        *value = I::try_from(buffer).map_err(|_| "something happened".to_owned())?;
        Ok(ret)
    }
    fn input_text_left(&self, label: &ImStr, buf: &mut ImString) {
        unsafe { imgui_sys::igAlignTextToFramePadding() };
        self.text(label);
        self.same_line(0.0);
        self.push_id(label);
        self.input_text(im_str!("###Input"), buf).build();
        self.pop_id();
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
            &items.iter().map(|item| item.as_ref()).collect::<Vec<_>>(),
            height_in_items,
        )
    }
}