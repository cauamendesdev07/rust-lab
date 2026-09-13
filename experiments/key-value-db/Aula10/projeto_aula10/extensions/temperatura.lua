local function is_valid_temperature(value)
    if type(value) ~= "string" then
        return false
    end

    if value == "" then
        return false
    end

    local number = tonumber(value)

    if number == nil then
        return false
    end

    if number < -273.15 then
        return false
    end

    return true
end

local function format_temperature(value)
    local celsius = tonumber(value)
    local fahrenheit = (celsius * 9 / 5) + 32

    return string.format(
        "Temperatura: %.2f °C | Convertida: %.2f °F",
        celsius,
        fahrenheit
    )
end

local function add(key, value)
    if not is_valid_temperature(value) then
        return {
            success = false,
            error = "Temperatura inválida"
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
        value = format_temperature(valor)
    }
end

return {
    prefix = "temp_",

    operations = {
        ADD = add,
        GET = get
    }
}