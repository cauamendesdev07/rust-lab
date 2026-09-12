use mlua::{Function, Lua};
use std::fs;

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

    pub fn transformar(&self, valor: String) -> mlua::Result<String> {
        let func: Function = self.lua
            .globals()
            .get("transformar")?;

        let resultado: String = func.call(valor)?;

        Ok(resultado)
    }

    pub fn registrar_funcoes(&self) -> mlua::Result<()> {
        let contar = self.lua.create_function(
            |_, valor: String| {
                Ok(valor.len())
            }
        )?;

        self.lua.globals().set("contar", contar)?;

        Ok(())
    }

    pub fn testar_rust(&self) -> mlua::Result<i64> {
        let func: Function = self.lua
            .globals()
            .get("testar_contar")?;

        let resultado: i64 = func.call(())?;

        Ok(resultado)
    }
}