pub struct Tema();

impl dialoguer::theme::Theme for Tema {
    fn format_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        write!(f, "{}", prompt)
    }

    fn format_error(&self, f: &mut dyn std::fmt::Write, err: &str) -> std::fmt::Result {
        write!(f, "error: {}", err)
    }

    fn format_confirm_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> std::fmt::Result {
        if !prompt.is_empty() {
            write!(f, "{} ", &prompt)?;
        }
        match default {
            None => write!(f, "[y/n] ")?,
            Some(true) => write!(f, "[Y/n] ")?,
            Some(false) => write!(f, "[y/N] ")?,
        }
        Ok(())
    }

    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> std::fmt::Result {
        let selection = selection.map(|b| if b { "yes" } else { "no" });

        match selection {
            Some(selection) if prompt.is_empty() => {
                write!(f, "{}", selection)
            }
            Some(selection) => {
                write!(f, "{} {}", &prompt, selection)
            }
            None if prompt.is_empty() => Ok(()),
            None => {
                write!(f, "{}", &prompt)
            }
        }
    }

    fn format_input_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> std::fmt::Result {
        match default {
            Some(default) if prompt.is_empty() => write!(f, "[{}]", default),
            Some(default) => write!(f, "{} [{}]", prompt, default),
            None => write!(f, "{}", prompt),
        }
    }

    fn format_input_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        write!(f, "{}{}", prompt, sel)
    }

    fn format_password_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_input_prompt(f, prompt, None)
    }

    fn format_password_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, "[hidden]")
    }

    fn format_select_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, sel)
    }

    fn format_multi_select_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_sort_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
        }
        Ok(())
    }

    fn format_sort_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        self.format_multi_select_prompt_selection(f, prompt, selections)
    }

    fn format_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        active: bool,
    ) -> std::fmt::Result {
        write!(f, "{} {}", if active { ">" } else { " " }, text)
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match (checked, active) {
                (true, true) => "> [x]",
                (true, false) => "  [x]",
                (false, true) => "> [ ]",
                (false, false) => "  [ ]",
            },
            text
        )
    }

    fn format_sort_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        picked: bool,
        active: bool,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match (picked, active) {
                (true, true) => "> [x]",
                (false, true) => "> [ ]",
                (_, false) => "  [ ]",
            },
            text
        )
    }
}