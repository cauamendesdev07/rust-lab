use mlua::{Function, Lua, Table, Value};

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::database::storage::Storage;

struct Extension {
    prefix: String,
    operations: HashMap<String, mlua::RegistryKey>,
}

pub struct LuaBridge {
    lua: Lua,
    extensions: Vec<Extension>,
}

impl LuaBridge {
    pub fn new() -> mlua::Result<Self> {
        Ok(Self {
            lua: Lua::new(),
            extensions: Vec::new(),
        })
    }

    pub fn load_extensions(&mut self, directory: &str) -> mlua::Result<()> {
        let entries = fs::read_dir(directory)
            .map_err(mlua::Error::external)?;

        for entry in entries {
            let entry = entry
                .map_err(mlua::Error::external)?;

            let path = entry.path();

            if path.extension().and_then(|extension| extension.to_str()) != Some("lua") {
                continue;
            }

            self.load_extension(&path)?;
        }

        Ok(())
    }

    fn load_extension(&mut self, path: &Path) -> mlua::Result<()> {
        let codigo = fs::read_to_string(path)
            .map_err(mlua::Error::external)?;

        let registration: Table = self
            .lua
            .load(&codigo)
            .eval()?;

        let prefix: String = registration.get("prefix")?;

        if prefix.trim().is_empty() {
            return Err(mlua::Error::runtime(
                "Extensão possui prefixo vazio",
            ));
        }

        let operations: Table = registration.get("operations")?;

        let mut registered_operations = HashMap::new();

        for pair in operations.pairs::<String, Value>() {
            let (operation, value) = pair?;

            let function = match value {
                Value::Function(function) => function,

                Value::String(function_name) => {
                    let function_name = function_name.to_str()?;

                    registration.get(function_name)?
                }

                _ => {
                    return Err(mlua::Error::runtime(
                        format!(
                            "Operação '{}' não possui uma função Lua válida",
                            operation
                        ),
                    ));
                }
            };

            let function_key = self
                .lua
                .create_registry_value(function)?;

            if registered_operations
                .insert(operation.clone(), function_key)
                .is_some()
            {
                return Err(mlua::Error::runtime(
                    format!(
                        "Operação duplicada na extensão: {}",
                        operation
                    ),
                ));
            }
        }

        if registered_operations.is_empty() {
            return Err(mlua::Error::runtime(
                "Extensão não possui operações",
            ));
        }

        for extension in &self.extensions {
            if extension.prefix == prefix {
                return Err(mlua::Error::runtime(
                    format!(
                        "Prefixo duplicado: {}",
                        prefix
                    ),
                ));
            }
        }

        self.extensions.push(Extension {
            prefix,
            operations: registered_operations,
        });

        Ok(())
    }

    fn find_extension(&self, key: &str) -> Option<&Extension> {
        self.extensions
            .iter()
            .find(|extension| key.starts_with(&extension.prefix))
    }

    fn execute_lua_operation(
        &self,
        operation: &str,
        key: &str,
        value: Option<&str>,
        storage: &Storage,
    ) -> mlua::Result<LuaOperationResult> {
        let extension = self
            .find_extension(key)
            .ok_or_else(|| {
                mlua::Error::runtime(
                    format!(
                        "Nenhuma extensão encontrada para a chave '{}'",
                        key
                    ),
                )
            })?;

        let function_key = extension
            .operations
            .get(operation)
            .ok_or_else(|| {
                mlua::Error::runtime(
                    format!(
                        "A extensão '{}' não suporta a operação '{}'",
                        extension.prefix,
                        operation
                    ),
                )
            })?;

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

            let function: Function =
                self.lua.registry_value(function_key)?;

            let resultado: Table = match operation {
                "ADD" => {
                    let value = value.ok_or_else(|| {
                        mlua::Error::runtime(
                            "ADD requer um valor",
                        )
                    })?;

                    function.call((
                        key.to_string(),
                        value.to_string(),
                    ))?
                }

                "GET" => {
                    function.call(key.to_string())?
                }

                _ => {
                    return Err(mlua::Error::runtime(
                        format!(
                            "Operação desconhecida: {}",
                            operation
                        ),
                    ));
                }
            };

            let success: bool =
                resultado.get("success")?;

            if success {
                let value: Option<String> =
                    resultado.get("value")?;

                Ok(LuaOperationResult {
                    success: true,
                    value,
                    error: None,
                })
            } else {
                let error: Option<String> =
                    resultado.get("error")?;

                Ok(LuaOperationResult {
                    success: false,
                    value: None,
                    error,
                })
            }
        })
    }

    pub fn execute_add(
        &self,
        key: &str,
        value: &str,
        storage: &Storage,
    ) -> mlua::Result<LuaOperationResult> {
        self.execute_lua_operation(
            "ADD",
            key,
            Some(value),
            storage,
        )
    }

    pub fn execute_get(
        &self,
        key: &str,
        storage: &Storage,
    ) -> mlua::Result<LuaOperationResult> {
        self.execute_lua_operation(
            "GET",
            key,
            None,
            storage,
        )
    }
}

pub struct LuaOperationResult {
    pub success: bool,
    pub value: Option<String>,
    pub error: Option<String>,
}
