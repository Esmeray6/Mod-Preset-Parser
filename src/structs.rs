use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use druid::commands;
use druid::{AppDelegate, Command, Data, DelegateCtx, Env, Handled, Lens, Target};
use scraper::{Html, Selector};

#[derive(Clone, Data, Lens, Debug)]
pub struct ModListInfo {
    pub mods: String,
}

#[derive(Clone, Data, Debug)]
pub struct MyDelegate {
    pub mod_list: Arc<Mutex<Vec<String>>>,
    pub dlc_prefixes: Arc<HashMap<String, String>>,
}

impl AppDelegate<ModListInfo> for MyDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx<'_>,
        _target: Target,
        cmd: &Command,
        data: &mut ModListInfo,
        _env: &Env,
    ) -> Handled {
        if cmd.is(commands::SAVE_FILE_AS) {
            let path_opt = cmd.get(commands::SAVE_FILE_AS);

            if let Some(path) = path_opt {
                let document = fs::read_to_string(path.path())
                    .expect("Error when reading the mod preset file");
                
                let mod_list_lock = &mut *self.mod_list.lock().unwrap();
                mod_list_lock.clear();

                let markup = Html::parse_document(&document);
                let mods_selector = Selector::parse(
                    "div.mod-list > table > tbody > tr > td[data-type='DisplayName']",
                )
                .expect("No mod list found");
                let dlc_selector = Selector::parse(
                    "div.dlc-list > table > tbody > tr > td[data-type='DisplayName']",
                )
                .expect("No mod list found");


                for element in markup.select(&dlc_selector) {
                    let inner_html = element.text().next().unwrap();
                    dbg!(&inner_html);
                    let dlc_prefix = self.dlc_prefixes.get(&*inner_html);
                    if let Some(dlc_name) = dlc_prefix {
                        mod_list_lock.push(dlc_name.to_string());
                    }
                }

                for element in markup.select(&mods_selector) {
                    let mut mod_name = format!("@{}", element.text().next().unwrap());
                    mod_name.retain(|c| c.is_alphanumeric() || c == '@');
                    mod_list_lock.push(mod_name);
                }

                mod_list_lock.sort_by(|a,b| {a.to_lowercase().cmp(&b.to_lowercase())});
                data.mods = mod_list_lock.join(";");
            }

            return Handled::Yes;
        }
        Handled::No
    }
}
