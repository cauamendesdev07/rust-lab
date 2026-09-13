local function add(key, value)
    if value == "" then
        return {
            success = false,
            error = "Valor vazio"
        }
    end

    return {
        success = true
    }
end

local function get(key)
    local valor = get_value(key)

    if valor == nil then
        return {
            success = false,
            error = "Chave não encontrada"
        }
    end

    return {
        success = true,
        value = valor
    }
end

return {
    prefix = "cpf_",

    operations = {
        ADD = add,
        GET = get
    }
}