use mlua::{Function, Lua};
use std::fs;
use crate::database::storage::Storage;

pub struct LuaBridge {
    lua: Lua,
}

impl LuaBridge {
    pub fn new() -> mlua::Result<Self> {
        Ok(Self {
            lua: Lua::new(),
        })
    }

    pub fn carregar_script(&self, caminho: &str) -> mlua::Result<()> {
        let codigo = fs::read_to_string(caminho)
            .map_err(mlua::Error::external)?;

        self.lua.load(&codigo).exec()?;

        Ok(())
    }

    pub fn executar_com_storage(
        &self,
        storage: &Storage,
    ) -> mlua::Result<String> {
        self.lua.scope(|scope| {
            let get_value = scope.create_function(
                |_, chave: String| {
                    match storage.get(&chave) {
                        Some(valor) => Ok(Some(valor.clone())),
                        None => Ok(None),
                    }
                }
            )?;

            let find_key_by_value = scope.create_function(
                |_, valor: String| {
                    match storage.find_key_by_value(&valor) {
                        Some(chave) => Ok(Some(chave.clone())),
                        None => Ok(None),
                    }
                }
            )?;

            let globals = self.lua.globals();

            globals.set("get_value", get_value)?;
            globals.set(
                "find_key_by_value",
                find_key_by_value,
            )?;

            let func: Function = globals.get("executar")?;

            let resultado: String = func.call(())?;

            Ok(resultado)
        })
    }

}